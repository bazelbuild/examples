use addr2line::Loader;
use fallible_iterator::FallibleIterator;
use findshlibs::{IterationControl, SharedLibrary, TargetSharedLibrary};

#[test]
#[allow(clippy::fn_to_numeric_cast)]
fn correctness() {
    let path = std::env::current_exe().unwrap();
    let ctx = Loader::new(&path).unwrap();
    let module_base = ctx.relative_address_base();

    let mut bias = None;
    TargetSharedLibrary::each(|lib| {
        bias = Some((lib.virtual_memory_bias().0 as u64).wrapping_sub(module_base));
        IterationControl::Break
    });

    #[allow(unused_mut)]
    let mut test = |sym: u64, expected_prefix: &str| {
        let ip = sym.wrapping_sub(bias.unwrap());

        let frames = ctx.find_frames(ip).unwrap();
        let frame = frames.last().unwrap().unwrap();
        let name = frame.function.as_ref().unwrap().demangle().unwrap();
        // Old rust versions generate DWARF with wrong linkage name,
        // so only check the start.
        if !name.starts_with(expected_prefix) {
            panic!("incorrect name '{}', expected {:?}", name, expected_prefix);
        }
    };

    test(test_function as u64, "correctness::test_function");
    test(
        small::test_function as u64,
        "correctness::small::test_function",
    );
    test(auxiliary::foo as u64, "auxiliary::foo");
}

mod small {
    pub fn test_function() {
        println!("y");
    }
}

fn test_function() {
    println!("x");
}

#[test]
fn zero_function() {
    let path = std::env::current_exe().unwrap();
    let ctx = Loader::new(&path).unwrap();
    for probe in 0..10 {
        assert!(ctx.find_frames(probe).unwrap().count().unwrap() < 10);
    }
}
