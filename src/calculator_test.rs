#[cfg(test)]
mod tests{
    use calculator::*;

    #[test]
    fn return_zero_for_empty_string(){ assert_eq!(0, add(""));}

    #[test]
    fn return_value_of_single_digit_string(){ assert_eq!(2, add("2"));}

    #[test]
    fn return_sum_of_two_comma_separated_values(){
        assert_eq!(3, add("1,2"));
    }

    #[test]
    fn return_sum_of__unknown_number_of_comma_separated_values(){
        assert_eq!(10, add("1,2,3,4"));
    }

    #[test]
    fn return_sum_of_newline_and_comma_separated_values(){
        assert_eq!(10, add("1,2\n3,4"));
    }

    #[test]
    fn return_sum_of_custom_delimiter_separated_values(){
        assert_eq!(10, add("//;\n1;2;3;4"));
    }

    #[test]
    fn return_sum_of_multiple_custom_delimiter_separated_values(){
        assert_eq!(10, add("//[;][*]\n1;2*3;4"));
    }

    #[test]
    #[should_panic(expected="Negatives not allowed: [-3, -4]")]
    fn throw_exception_for_negatives(){
        assert_eq!(10, add("1,2,-3,-4"));
    }

    #[test]
    fn filter_out_numbers_larger_than_1000(){
        assert_eq!(10, add("1,1001,2,3,4,2000"));
    }

    #[test]
    fn return_sum_of_custom_multi_length_delimiter_separated_values(){
        assert_eq!(10, add("//[;;;]\n1;;;2;;;3;;;4"));
    }

    #[test]
    fn return_sum_of_multiple_custom_multi_length_delimiter_separated_values(){
        assert_eq!(10, add("//[;;;][***]\n1;;;2***3;;;4"));
    }

}