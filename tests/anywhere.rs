use bpaf::*;

#[test]
fn parse_anywhere_positional() {
    let a = any::<String>("x")
        .guard(|h| h != "--help", "ignore help")
        .anywhere();

    let b = short('b').switch();
    let parser = construct!(a, b).to_options();

    let r = parser
        .run_inner(Args::from(&["--help"]))
        .unwrap_err()
        .unwrap_stdout();

    assert_eq!(
        r,
        "Usage: <x> [-b]\n\nAvailable options:\n    -b\n    -h, --help  Prints help information\n"
    );
    // this should be allowed because "anywhere" prevents anything inside from being positional
    parser.check_invariants(true);
}

#[test]
fn parse_anywhere_no_catch() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    // Usage: -a <x> [-c],

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");
}

#[test]
fn anywhere_catch_optional() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch().optional();
    let bc = short('a').switch();
    let parser = construct!(ab, bc).to_options();

    let r = parser.run_inner(Args::from(&["-a", "10"])).unwrap();
    assert_eq!(r, (Some(((), 10)), false));

    let r = parser.run_inner(Args::from(&["-a"])).unwrap();
    assert_eq!(r, (None, true));

    let r = parser.run_inner(Args::from(&[])).unwrap();
    assert_eq!(r, (None, false));
}

#[test]
fn anywhere_catch_many() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch().many();
    let bc = short('a').switch();
    let parser = construct!(ab, bc).to_options();

    let r = parser.run_inner(Args::from(&["-a", "10"])).unwrap();
    assert_eq!(r, (vec![((), 10)], false));

    let r = parser.run_inner(Args::from(&["-a"])).unwrap();
    assert_eq!(r, (Vec::new(), true));

    let r = parser.run_inner(Args::from(&[])).unwrap();
    assert_eq!(r, (Vec::new(), false));
}

#[test]
fn anywhere_catch_fallback() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch().fallback(((), 10));
    let bc = short('a').switch();
    let parser = construct!(ab, bc).to_options();

    let r = parser.run_inner(Args::from(&["-a", "12"])).unwrap();
    assert_eq!(r, (((), 12), false));

    let r = parser.run_inner(Args::from(&["-a"])).unwrap();
    assert_eq!(r, (((), 10), true));

    let r = parser.run_inner(Args::from(&[])).unwrap();
    assert_eq!(r, (((), 10), false));
}

#[test]
fn parse_anywhere_catch_required() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    // this should complain about unexpected -a
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Couldn't parse \"221b\": invalid digit found in string");

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "Expected <x>, pass --help for usage information");
}

#[test]
fn parse_anywhere_catch_optional() {
    let a = short('a').req_flag(());
    let b = positional::<usize>("x");
    let ab = construct!(a, b).anywhere().catch().optional();
    let c = short('c').switch();
    let parser = construct!(ab, c).to_options();

    let r = parser
        .run_inner(Args::from(&["3", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "3 is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-c", "-a"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-c", "-a", "221b"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");

    let r = parser
        .run_inner(Args::from(&["-a", "221b", "-c"]))
        .unwrap_err()
        .unwrap_stderr();
    assert_eq!(r, "-a is not expected in this context");
}

#[test]
fn anywhere_literal() {
    let tag = any::<String>("-mode").guard(|x| x == "-mode", "not mode");
    let mode = positional::<usize>("value");
    let a = construct!(tag, mode).anywhere().catch().many();
    let b = short('b').switch();
    let parser = construct!(a, b).to_options();

    let r = parser
        .run_inner(Args::from(&["-b", "-mode", "12"]))
        .unwrap();
    assert_eq!(r, (vec![("-mode".to_owned(), 12)], true));

    let r = parser
        .run_inner(Args::from(&["-mode", "12", "-b"]))
        .unwrap();
    assert_eq!(r, (vec![("-mode".to_owned(), 12)], true));

    let r = parser.run_inner(Args::from(&["-mode", "12"])).unwrap();
    assert_eq!(r, (vec![("-mode".to_owned(), 12)], false));
}
