use std::cell::RefCell;

use crate::prelude::*;

day!(7);

mod dirs {
    use std::{
        cell::RefCell,
        collections::HashMap,
        ops::Deref,
        rc::{Rc, Weak},
    };

    use itertools::Itertools;

    #[derive(Clone)]
    pub struct Dir {
        inner: Rc<RefCell<DirInner>>,
    }

    type WeakDir = Weak<RefCell<DirInner>>;

    pub struct DirInner {
        files: HashMap<Vec<u8>, u64>,
        subdirs: HashMap<Vec<u8>, Dir>,
        parent: WeakDir,
    }

    impl DirInner {
        pub fn new(parent: WeakDir) -> Self {
            Self {
                files: HashMap::new(),
                subdirs: HashMap::new(),
                parent,
            }
        }

        pub fn direct_size(&self) -> u64 {
            self.files.values().sum()
        }

        pub fn recursive_size(&self) -> u64 {
            self.direct_size()
                + self
                    .subdirs
                    .values()
                    .map(|dir| dir.borrow().recursive_size())
                    .sum::<u64>()
        }

        pub fn add_file(&mut self, name: Vec<u8>, size: u64) {
            self.files.insert(name, size);
        }

        pub fn parent(&self) -> Option<Dir> {
            self.parent.upgrade().map(|inner| Dir { inner })
        }

        pub fn iter_subdirs(&self) -> impl Iterator<Item = &Dir> {
            self.subdirs.values()
        }

        // panics if not found :(
        pub fn get_subdir(&self, name: &[u8]) -> Dir {
            self.subdirs
                .get(name)
                .expect("Couldn't find subdir")
                .clone()
        }
    }

    impl Dir {
        pub fn new(inner: DirInner) -> Self {
            Self {
                inner: Rc::new(RefCell::new(inner)),
            }
        }

        pub fn add_subdir(&self, name: Vec<u8>) {
            let newdir = Dir::new(DirInner::new(Rc::downgrade(&self.inner)));
            self.inner.borrow_mut().subdirs.insert(name, newdir);
        }

        pub fn root() -> Self {
            Self::new(DirInner::new(WeakDir::new()))
        }

        pub fn display(&self) -> DirPrinter<'_> {
            DirPrinter {
                dir: self,
                depth: 0,
            }
        }
    }

    impl Deref for Dir {
        type Target = RefCell<DirInner>;

        fn deref(&self) -> &Self::Target {
            self.inner.deref()
        }
    }

    pub struct DirPrinter<'dir> {
        dir: &'dir Dir,
        depth: u8,
    }

    impl<'dir> DirPrinter<'dir> {
        pub fn indent(&self) -> impl std::fmt::Display + '_ {
            std::iter::repeat("  ")
                .take(usize::from(self.depth))
                .format("")
        }
    }

    impl<'dir> std::fmt::Display for DirPrinter<'dir> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::str::from_utf8;
            let inner = self.dir.borrow();

            writeln!(f, "{}.", self.indent())?;

            for (filename, size) in inner.files.iter() {
                let filename = from_utf8(filename).unwrap();
                writeln!(f, "{}{size} {filename}", self.indent())?;
            }

            for (name, subdir) in &inner.subdirs {
                let name = from_utf8(name).unwrap();
                writeln!(f, "{}{name}/", self.indent())?;
                write!(
                    f,
                    "{}",
                    DirPrinter {
                        dir: subdir,
                        depth: self.depth + 1,
                    }
                )?;
            }

            Ok(())
        }
    }
}

pub(self) use dirs::Dir;

mod parsing {
    use std::{cell::RefCell, str::from_utf8};

    use crate::utils::eof_iterator;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till},
        character::{
            complete::{newline, u64 as nom_u64},
            is_newline,
        },
        combinator::{map, map_parser},
        error::Error,
        sequence::{delimited, preceded, separated_pair, terminated},
        Finish, IResult,
    };

    use super::Dir;

    pub fn parse_input(input: &[u8]) -> Result<Dir, String> {
        let root = Dir::root();
        match parse_input_inner(input, root).finish() {
            Ok((_, root)) => Ok(root),
            Err(err) => Err(format!(
                "{:?}",
                Error::new(from_utf8(err.input).unwrap(), err.code)
            )),
        }
    }

    fn parse_input_inner(input: &[u8], root: Dir) -> IResult<&[u8], Dir> {
        let cwd = RefCell::new(root.clone());
        let (input, _) = terminated(tag(&b"$ cd /"[..]), newline)(input)?;
        let mut it = eof_iterator(input, |s| do_command(s, &cwd));
        () = it.collect();
        let (s, ()) = it.finish()?;
        Ok((s, root))
    }

    fn do_command<'inp, 'dir>(
        input: &'inp [u8],
        cwd: &'dir RefCell<Dir>,
    ) -> IResult<&'inp [u8], ()> {
        let do_cd = |name: &[u8]| {
            let newdir = {
                let inner = cwd.borrow();
                let inner = inner.borrow();
                if name == b".." {
                    inner.parent().expect("No parent")
                } else {
                    inner.get_subdir(name)
                }
            };
            cwd.replace(newdir);
        };

        let (input, ()) = preceded(
            tag(&b"$ "[..]),
            alt((
                map(
                    delimited(tag(&b"cd "[..]), take_till(is_newline), newline),
                    do_cd,
                ),
                map_parser(
                    preceded(
                        terminated(tag(&b"ls"[..]), newline),
                        take_till(|b| b == b'$'),
                    ),
                    |s| ls_body(s, cwd),
                ),
            )),
        )(input)?;

        Ok((input, ()))
    }

    fn ls_body<'inp, 'dir>(input: &'inp [u8], cwd: &'dir RefCell<Dir>) -> IResult<&'inp [u8], ()> {
        let add_subdir = |name: &[u8]| cwd.borrow().add_subdir(name.to_owned());
        let add_file = |(size, name): (u64, &[u8])| {
            cwd.borrow_mut()
                .borrow_mut()
                .add_file(name.to_owned(), size)
        };

        let mut iter = eof_iterator(
            input,
            terminated(
                alt((
                    map(
                        preceded(tag(&b"dir "[..]), take_till(is_newline)),
                        add_subdir,
                    ),
                    map(
                        separated_pair(nom_u64, tag(&b" "[..]), take_till(is_newline)),
                        add_file,
                    ),
                )),
                newline,
            ),
        );

        () = iter.collect();
        iter.finish()
    }
}

const PART1_THRESHOLD: u64 = 100_000;

fn count_part1(dir: &Dir) -> u64 {
    let inner = RefCell::borrow(dir);
    let mut total = 0;
    let size = inner.recursive_size();
    if size <= PART1_THRESHOLD {
        total += size;
    }

    total + inner.iter_subdirs().map(count_part1).sum::<u64>()
}

const TOTAL_SPACE: u64 = 70_000_000;
const UPDATE_SIZE: u64 = 30_000_000;

fn find_part2(dir: &Dir) -> u64 {
    fn part2_inner(dir: &Dir, min: &mut u64, needed: u64) -> u64 {
        let inner = dir.borrow();

        let fsize = inner.direct_size();

        let sub_size: u64 = inner
            .iter_subdirs()
            .map(|sub| part2_inner(sub, min, needed))
            .sum();

        let tot_size = fsize + sub_size;

        if tot_size >= needed && tot_size < *min {
            *min = tot_size;
        }

        tot_size
    }

    let needed = UPDATE_SIZE - (TOTAL_SPACE - dir.borrow().recursive_size());
    let mut min = u64::MAX;
    part2_inner(dir, &mut min, needed);
    min
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed loading input file");
    let root = whatever!(parsing::parse_input(&input), "Failed parsing input");

    println!(
        "Finished parsing root:\n----- DIR -----\n{}---------------\n",
        root.display()
    );

    let part1: u64 = count_part1(&root);
    println!("Part 1: {part1}");

    let part2 = find_part2(&root);
    println!("Part 2: {part2}");

    Ok(())
}
