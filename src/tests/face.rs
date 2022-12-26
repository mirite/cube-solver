use crate::cube_parts::face::{rotate_face_clockwise, rotate_face_counter_clockwise, Face};
use crate::cube_parts::row::Row;

fn get_test_face() -> Face {
    Face {
        t: Row {
            l: 'a',
            c: 'b',
            r: 'c',
        },
        m: Row {
            l: 'd',
            c: 'e',
            r: 'f',
        },
        b: Row {
            l: 'g',
            c: 'h',
            r: 'i',
        },
    }
}

#[test]
fn clockwise_single() {
    let expected_result = Face {
        t: Row {
            l: 'g',
            c: 'd',
            r: 'a',
        },
        m: Row {
            l: 'h',
            c: 'e',
            r: 'b',
        },
        b: Row {
            l: 'i',
            c: 'f',
            r: 'c',
        },
    };
    let mut face = get_test_face();
    face = rotate_face_clockwise(face);
    assert_eq!(
        face, expected_result,
        "\nActual Face:\n{}\n\nExpected Face:\n{}\n",
        face, expected_result
    );
}

#[test]
fn counter_clockwise_single() {
    let expected_result = Face {
        t: Row {
            l: 'c',
            c: 'f',
            r: 'i',
        },
        m: Row {
            l: 'b',
            c: 'e',
            r: 'h',
        },
        b: Row {
            l: 'a',
            c: 'd',
            r: 'g',
        },
    };
    let mut face = get_test_face();
    face = rotate_face_counter_clockwise(face);
    assert_eq!(
        face, expected_result,
        "\nActual Face:\n{}\n\nExpected Face:\n{}\n",
        face, expected_result
    );
}
