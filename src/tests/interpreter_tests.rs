use crate::canvas::Canvas;
use crate::tests::PATH;
use std::sync::LazyLock;

static TEST_CANVAS: LazyLock<Canvas> = LazyLock::new(|| Canvas::new(&(PATH.to_owned() + "interpreter_tests.png"), 1).expect("Failed to build test canvas"));

mod pointer_movement {
    use crate::canvas::Coordinate;
    use crate::interpreter::{CC, CodeState, DP, Interpreter, PointerAspect};
    use crate::tests::interpreter_tests::TEST_CANVAS;

    #[test]
    fn can_step() {
        let mut interpreter = Interpreter::new();
        interpreter.update_coordinate((1,1));
        let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
        assert!(matches!(state, CodeState::Continue(_)))
    }

    #[test]
    fn updates_coordinate() {
        let mut interpreter = Interpreter::new();
        interpreter.update_coordinate((1,1));
        interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
        let new_coordinate = interpreter.current_coordinate();
        assert_eq!(new_coordinate, (2,1))  
    }

    #[test]
    fn simple_cc_choice() {
        let mut interpreter = Interpreter::new();
        interpreter.update_coordinate((1,1));
        interpreter.modify_pointer(PointerAspect::CC);
        interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
        let new_coordinate = interpreter.current_coordinate();
        assert_eq!(new_coordinate, (2,2))          
    }

    #[test]
    fn complex_pointer_choice() {
        let mut new_coordinates: [Coordinate; 2] = [(0,0),(0,0)];
        for cc in [CC::Left, CC::Right] {
            let mut interpreter = Interpreter::from_data(
                DP::East,
                cc,
                (4,1),
                0,
                false
            );
            interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
            new_coordinates[cc as usize] = interpreter.current_coordinate();
        }
        assert_eq!(new_coordinates, [(6,1), (6,3)])
    }

    #[test]
    fn very_complex_pointer_choice() {
        let mut interpreter = Interpreter::new();
        interpreter.update_coordinate((13,1));
        interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
        assert_eq!(interpreter.current_coordinate(), (17,7))
    }

    #[test]
    fn dp_and_cc_determine_step() {
        let mut new_coordinates: [[Coordinate; 4];2] = [
            [(0,0), (0,0), (0,0), (0,0)],
            [(0,0), (0,0), (0,0), (0,0)]
            ];
        for dp in [DP::North, DP::East, DP::South, DP::West] {
            for cc in [CC::Left, CC::Right] {
                let mut interpreter = Interpreter::from_data(
                    dp,
                    cc,
                    (9,2),
                    0,
                    false
                );
                interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
                new_coordinates[cc as usize][dp as usize] = interpreter.current_coordinate();
            }
        }
        assert_eq!(new_coordinates, [
            [(9,1),(11,2),(10,4),(8,3)],
            [(10,1),(11,3),(9,4),(8,2)]
            ]) 
    }

    mod through_white {
        use crate::interpreter::{CC, CodeState, DP, Interpreter, PointerAspect};
        use crate::tests::interpreter_tests::TEST_CANVAS;
        #[test]
        fn can_step() {
            let mut interpreter = Interpreter::new();
            interpreter.update_coordinate((1,4));
            let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
            assert!(matches!(state, CodeState::Continue(None)))
        }
        
        #[test]
        fn turns_at_black_and_modifies_pointer() {
            let mut interpreter = Interpreter::new();
            interpreter.update_coordinate((1,6));
            interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
            assert!(matches!(interpreter.cc(), CC::Right));
            assert!(matches!(interpreter.dp(), DP::South));
            assert_eq!(interpreter.current_coordinate(), (2,7)) 
        }

        #[test]
        fn terminates_if_retreads_path() {
            let mut interpreter = Interpreter::new();
            interpreter.update_coordinate((9,6));
            let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
            assert!(matches!(state, CodeState::Terminate));
        }

        #[test]
        fn can_incidentally_retread_path() {
            let mut interpreter = Interpreter::new();
            interpreter.update_coordinate((4,6));
            let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
            assert!(matches!(state, CodeState::Continue(_)));
        }
    }
}

mod piet_commands {
    use crate::canvas::Coordinate;
    use crate::interpreter::{CodeState, Interpreter, PietCommand, PointerAspect};
    use crate::tests::interpreter_tests::TEST_CANVAS;

    #[test]
    fn step_from_colour_to_white_has_no_command() {
        let mut interpreter = Interpreter::new();
        interpreter.update_coordinate((19,1));
        let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
        assert!(matches!(state, CodeState::Continue(None)))
    }

    #[test]
    fn obtains_correct_commands_when_stepping_between_colours() {
        let starting_coords: [[Coordinate; 3]; 6] = [
            [(19,1), (22,1), (25,1)],
            [(19,3), (22,3), (25,3)],
            [(19,5), (22,5), (25,5)],
            [(19,7), (22,7), (25,7)],
            [(19,9), (22,9), (25,9)],
            [(19,11), (22,11), (25,11)],
            ];
        let commands = [
            [PietCommand::Push(1), PietCommand::Push(1), PietCommand::Pop],
            [PietCommand::Add,PietCommand::Subtract,PietCommand::Multiply],
            [PietCommand::Divide,PietCommand::Modulo,PietCommand::Not],
            [PietCommand::Greater,PietCommand::Pointer,PietCommand::Switch],
            [PietCommand::Duplicate,PietCommand::Roll,PietCommand::InputNum],
            [PietCommand::InputChar,PietCommand::OutputNum,PietCommand::OutputChar]
            ];
        for i in 0..6 {
            for j in 0..3 {
                if (i,j) == (0,0) {
                    continue // handled in different test
                }
                let mut interpreter = Interpreter::new();
                interpreter.update_coordinate(starting_coords[i][j]);
                let state = interpreter.get_next_state(&*TEST_CANVAS, PointerAspect::CC);
                match state {
                    CodeState::Continue(Some(command)) => {
                        assert_eq!(command, commands[i][j]);
                    }
                    _ => panic!()
                }
            }        
        }
    }

}

mod stackops {
    use crate::interpreter::{CC, DP, Interpreter, commands};

    #[test]
    fn basic_operations_work() {
        let push_stack =  &mut Vec::new();
        commands::push(push_stack, 10).expect("");
        assert_eq!(*push_stack, vec![10]);

        let pop_stack =  &mut vec![13, 10];
        commands::pop(pop_stack).expect("");
        assert_eq!(*pop_stack, vec![13]);

        let add_stack =  &mut vec![13, 10];
        commands::add(add_stack).expect("");
        assert_eq!(*add_stack, vec![23]);

        let subtract_stack =  &mut vec![13, 10];
        commands::subtract(subtract_stack).expect("");
        assert_eq!(*subtract_stack, vec![3]);

        let multiply_stack =  &mut vec![13, 10];
        commands::multiply(multiply_stack).expect("");
        assert_eq!(*multiply_stack, vec![130]);

        let divide_stack =  &mut vec![13, 10];
        commands::divide(divide_stack).expect("");
        assert_eq!(*divide_stack, vec![1]);

        let modulo_stack =  &mut vec![13, 10];
        commands::modulo(modulo_stack).expect("");
        assert_eq!(*modulo_stack, vec![3]);

        let not_stack =  &mut vec![13, 10];
        commands::not(not_stack).expect("");
        assert_eq!(*not_stack, vec![13, 0]);

        let greater_stack =  &mut vec![13, 10];
        commands::greater(greater_stack).expect("");
        assert_eq!(*greater_stack, vec![1]);

        let duplicate_stack =  &mut vec![13, 10];
        commands::duplicate(duplicate_stack).expect("");
        assert_eq!(*duplicate_stack, vec![13, 10, 10]);
    }

    #[test]
    fn io_works() {
        let stack = &mut Vec::new();
        commands::input_char(stack, 'a');
        assert_eq!(commands::output_char(stack).expect(""), 'a');

        commands::input_num(stack, 7);
        assert_eq!(commands::output_num(stack).expect(""), 7);
    }

    #[test]
    fn pointer_changes_work() {
        let mut interpreter = Interpreter::new();
        interpreter.stack().push(2);
        commands::pointer(&mut interpreter).expect("");
        assert_eq!(interpreter.dp(), DP::West);

        interpreter.stack().push(3);
        commands::switch(&mut interpreter).expect("");
        assert_eq!(interpreter.cc(), CC::Right)
    }

    #[test]
    fn roll_works() {
        let stack = &mut vec![10, 20, 15, 11, 3, 2];
        commands::roll(stack).expect("");
        assert_eq!(*stack, vec![10, 11, 20, 15])
    }

}
