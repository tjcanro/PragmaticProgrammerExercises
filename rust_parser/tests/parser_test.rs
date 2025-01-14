use rust_parser::parser::time_parser;

macro_rules! hr {
  ($x:expr) => {
      $x * 60
  };
}

macro_rules! pm {
  ($x:expr) => {
      $x + (12*60)
  };
}

#[test]
fn first_sanity_check() {
  assert_eq!(4, 4);
}

#[test]
fn sanity_check_list() {
  assert_eq!(time_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
}

#[test]
fn time_format_one() {
  assert_eq!(time_parser::time("11:30 am"), Ok(hr!(11) + 30));
}

#[test]
fn single_digit_format_one() {
  assert_eq!(time_parser::time("1:30 pm"), Ok(pm!(hr!(1)) + 30));
}

#[test]
fn time_format_two() {
  assert_eq!(time_parser::time("22:00"), Ok(hr!(22)));
}

#[test]
fn time_format_three() {
  assert_eq!(time_parser::time("10 pm"), Ok(pm!(hr!(10))));
}