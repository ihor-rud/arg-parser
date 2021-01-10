use main::*;

#[cfg(test)]
mod tests {

    mod test_bool {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as bool where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        fn test_parse_nothing() {
            let args = setup![];
            assert_eq!(args.val, false);
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, true);
        }

        #[test]
        fn test_parse_true() {
            let args = setup!["-q", "true"];
            assert_eq!(args.val, true);
        }

        #[test]
        fn test_parse_false() {
            let args = setup!["-q", "false"];
            assert_eq!(args.val, false);
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "true", "true"];
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "true", "--val", "true"];
            assert_eq!(args.val, true);
        }
    }

    mod test_option_bool {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Option<bool> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        fn test_parse_nothing() {
            let args = setup![];
            assert_eq!(args.val, None);
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, Some(true));
        }

        #[test]
        fn test_parse_true() {
            let args = setup!["-q", "true"];
            assert_eq!(args.val, Some(true));
        }

        #[test]
        fn test_parse_false() {
            let args = setup!["-q", "false"];
            assert_eq!(args.val, Some(false));
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "true", "true"];
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "true", "--val", "true"];
            assert_eq!(args.val, Some(true));
        }
    }

    mod test_vector_bool {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Vec<bool> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        #[should_panic]
        fn test_parse_nothing() {
            setup![];
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, []);
        }

        #[test]
        fn test_parse_true() {
            let args = setup!["-q", "true"];
            assert_eq!(args.val, [true]);
        }

        #[test]
        fn test_parse_false() {
            let args = setup!["-q", "false"];
            assert_eq!(args.val, [false]);
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        fn test_parse_many() {
            let args = setup!["-q", "true", "false"];
            assert_eq!(args.val, [true, false]);
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "true", "--val", "true"];
            assert_eq!(args.val, [true, true]);
        }
    }

    mod test_i32 {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as i32 where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        #[should_panic]
        fn test_parse_nothing() {
            setup![];
        }

        #[test]
        #[should_panic]
        fn test_parse_name_only() {
            setup!["-q"];
        }

        #[test]
        fn test_parse_number() {
            let args = setup!["-q", "100"];
            assert_eq!(args.val, 100);
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "100", "-7"];
        }

        #[test]
        #[should_panic]
        fn test_merge() {
            setup!["-q", "100", "--val", "-7"];
        }
    }

    mod test_option_i32 {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Option<i32> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        fn test_parse_nothing() {
            let args = setup![];
            assert_eq!(args.val, None);
        }

        #[test]
        #[should_panic]
        fn test_parse_name_only() {
            setup!["-q"];
        }

        #[test]
        fn test_parse_number() {
            let args = setup!["-q", "100"];
            assert_eq!(args.val, Some(100));
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "100", "-7"];
        }

        #[test]
        #[should_panic]
        fn test_merge() {
            setup!["-q", "100", "--val", "-7"];
        }
    }

    mod test_vector_i32 {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Vec<i32> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        #[should_panic]
        fn test_parse_nothing() {
            setup![];
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, []);
        }

        #[test]
        fn test_parse_number() {
            let args = setup!["-q", "100"];
            assert_eq!(args.val, [100]);
        }

        #[test]
        #[should_panic]
        fn test_parse_error() {
            setup!["-q", "foobar"];
        }

        #[test]
        fn test_parse_many() {
            let args = setup!["-q", "100", "-7"];
            assert_eq!(args.val, [100, -7]);
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "100", "--val", "-7"];
            assert_eq!(args.val, [100, -7]);
        }
    }

    mod test_string {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as String where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        #[should_panic]
        fn test_parse_nothing() {
            setup![];
        }

        #[test]
        #[should_panic]
        fn test_parse_name_only() {
            setup!["-q"];
        }

        #[test]
        fn test_parse_value() {
            let args = setup!["-q", "foo"];
            assert_eq!(args.val, "foo");
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "foo", "bar"];
        }

        #[test]
        #[should_panic]
        fn test_merge() {
            setup!["-q", "foo", "--val", "bar"];
        }
    }

    mod test_option_string {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Option<String> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        fn test_parse_nothing() {
            let args = setup![];
            assert_eq!(args.val, None);
        }

        #[test]
        #[should_panic]
        fn test_parse_name_only() {
            setup!["-q"];
        }

        #[test]
        fn test_parse_value() {
            let args = setup!["-q", "foo"];
            assert_eq!(args.val, Some("foo".to_string()));
        }

        #[test]
        #[should_panic]
        fn test_parse_many() {
            setup!["-q", "foo", "bar"];
        }

        #[test]
        #[should_panic]
        fn test_merge() {
            setup!["-q", "foo", "--val", "bar"];
        }
    }

    mod test_vector_string {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Vec<String> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        #[should_panic]
        fn test_parse_nothing() {
            setup![];
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, [] as [String; 0]);
        }

        #[test]
        fn test_parse_value() {
            let args = setup!["-q", "foo"];
            assert_eq!(args.val, ["foo".to_string()]);
        }

        #[test]
        fn test_parse_many() {
            let args = setup!["-q", "foo", "bar"];
            assert_eq!(args.val, ["foo".to_string(), "bar".to_string()]);
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "foo", "--val", "bar"];
            assert_eq!(args.val, ["foo".to_string(), "bar".to_string()]);
        }
    }

    mod test_option_vector {

        use crate::*;

        macro_rules! setup {
            ($($x:expr),*) => {{
                let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
                command_args_parser! {@private val as Option<Vec<String>> where {short: "q", long: "val"} strings.into_iter()}
            }};
        }

        #[test]
        fn test_parse_nothing() {
            let args = setup![];
            assert_eq!(args.val, None);
        }

        #[test]
        fn test_parse_name_only() {
            let args = setup!["-q"];
            assert_eq!(args.val, Some(vec![]));
        }

        #[test]
        fn test_parse_value() {
            let args = setup!["-q", "foo"];
            assert_eq!(args.val, Some(vec!["foo".to_string()]));
        }

        #[test]
        fn test_parse_many() {
            let args = setup!["-q", "foo", "bar"];
            assert_eq!(args.val, Some(vec!["foo".to_string(), "bar".to_string()]));
        }

        #[test]
        fn test_merge() {
            let args = setup!["-q", "foo", "--val", "bar"];
            assert_eq!(args.val, Some(vec!["foo".to_string(), "bar".to_string()]));
        }
    }

    use crate::*;

    macro_rules! setup {
        ($($x:expr),*) => {{
            let strings = vec!["main.rs".to_string(), $($x.to_string()),*];
            command_args_parser! {@private
                val1 as String where {short: "q", long: "val"};
                val2 as i32 where {short: "w", long: "val2"};
                val3 as Vec<bool> where {short: "e", long: "val3"}
                strings.into_iter()
            }
        }};
    }

    #[test]
    fn test_multiple_args() {
        let args = setup!["-q", "foo", "--val2", "100", "-e", "true", "true", "false"];
        assert_eq!(args.val1, "foo");
        assert_eq!(args.val2, 100);
        assert_eq!(args.val3, [true, true, false]);
    }
}
