#![allow(dead_code)]
//! Some helper structs/functions taken from bracketlib

use std::collections::HashSet;
use std::convert::TryInto;
use std::ops;

use bracket_pathfinding::prelude::Point;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}
impl Default for Rect {
    fn default() -> Rect {
        Rect::zero()
    }
}

impl Rect {
    // Create a new rectangle, specifying X/Y Width/Height
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Rect
    where
        T: TryInto<i32>,
    {
        let x_i32: i32 = x.try_into().ok().unwrap();
        let y_i32: i32 = y.try_into().ok().unwrap();
        Rect {
            x1: x_i32,
            y1: y_i32,
            x2: x_i32 + w.try_into().ok().unwrap(),
            y2: y_i32 + h.try_into().ok().unwrap(),
        }
    }

    // Create a new rectangle, specifying exact dimensions
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Rect
    where
        T: TryInto<i32>,
    {
        Rect {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }

    // Creates a zero rectangle
    pub fn zero() -> Rect {
        Rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Returns the center of the rectangle
    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    // Returns true if a point is inside the rectangle
    pub fn point_in_rect(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    // Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f(Point::new(x, y));
            }
        }
    }

    // Gets a set of all tiles in the rectangle
    pub fn point_set(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                result.insert(Point::new(x, y));
            }
        }
        result
    }

    // Returns the rectangle's width
    pub fn width(&self) -> i32 {
        i32::abs(self.x2 - self.x1)
    }

    // Returns the rectangle's height
    pub fn height(&self) -> i32 {
        i32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<Rect> for Rect {
    type Output = Rect;
    fn add(mut self, rhs: Rect) -> Rect {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}

pub fn to_cp437(c: char) -> usize {
    match c {
        '???' => 1,
        '???' => 2,
        '???' => 3,
        '???' => 4,
        '???' => 5,
        '???' => 6,
        '???' => 7,
        '???' => 8,
        '???' => 9,
        '???' => 10,
        '???' => 11,
        '???' => 12,
        '???' => 13,
        '???' => 14,
        '???' => 15,

        '???' => 16,
        '???' => 17,
        '???' => 18,
        '???' => 19,
        '??' => 20,
        '??' => 21,
        '???' => 22,
        '???' => 23,
        '???' => 24,
        '???' => 25,
        '???' => 26,
        '???' => 27,
        '???' => 28,
        '???' => 29,
        '???' => 30,
        '???' => 31,

        ' ' => 32,
        '!' => 33,
        '"' => 34,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '\'' => 39,
        '(' => 40,
        ')' => 41,
        '*' => 42,
        '+' => 43,
        ',' => 44,
        '-' => 45,
        '.' => 46,
        '/' => 47,

        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ':' => 58,
        ';' => 59,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,

        '@' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,

        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        '[' => 91,
        '\\' => 92,
        ']' => 93,
        '^' => 94,
        '_' => 95,

        '`' => 96,
        'a' => 97,
        'b' => 98,
        'c' => 99,
        'd' => 100,
        'e' => 101,
        'f' => 102,
        'g' => 103,
        'h' => 104,
        'i' => 105,
        'j' => 106,
        'k' => 107,
        'l' => 108,
        'm' => 109,
        'n' => 110,
        'o' => 111,

        'p' => 112,
        'q' => 113,
        'r' => 114,
        's' => 115,
        't' => 116,
        'u' => 117,
        'v' => 118,
        'w' => 119,
        'x' => 120,
        'y' => 121,
        'z' => 122,
        '{' => 123,
        '|' => 124,
        '}' => 125,
        '~' => 126,
        '???' => 127,

        '??' => 128,
        '??' => 129,
        '??' => 130,
        '??' => 131,
        '??' => 132,
        '??' => 133,
        '??' => 134,
        '??' => 135,
        '??' => 136,
        '??' => 137,
        '??' => 138,
        '??' => 139,
        '??' => 140,
        '??' => 141,
        '??' => 142,
        '??' => 143,

        '??' => 144,
        '??' => 145,
        '??' => 146,
        '??' => 147,
        '??' => 148,
        '??' => 149,
        '??' => 150,
        '??' => 151,
        '??' => 152,
        '??' => 153,
        '??' => 154,
        '??' => 155,
        '??' => 156,
        '??' => 157,
        '???' => 158,
        '??' => 159,

        '??' => 160,
        '??' => 161,
        '??' => 162,
        '??' => 163,
        '??' => 164,
        '??' => 165,
        '??' => 166,
        '??' => 167,
        '??' => 168,
        '???' => 169,
        '??' => 170,
        '??' => 171,
        '??' => 172,
        '??' => 173,
        '??' => 174,
        '??' => 175,

        '???' => 176,
        '???' => 177,
        '???' => 178,
        '???' => 179,
        '???' => 180,
        '???' => 181,
        '???' => 182,
        '???' => 183,
        '???' => 184,
        '???' => 185,
        '???' => 186,
        '???' => 187,
        '???' => 188,
        '???' => 189,
        '???' => 190,
        '???' => 191,

        '???' => 192,
        '???' => 193,
        '???' => 194,
        '???' => 195,
        '???' => 196,
        '???' => 197,
        '???' => 198,
        '???' => 199,
        '???' => 200,
        '???' => 201,
        '???' => 202,
        '???' => 203,
        '???' => 204,
        '???' => 205,
        '???' => 206,
        '???' => 207,

        '???' => 208,
        '???' => 209,
        '???' => 210,
        '???' => 211,
        '???' => 212,
        '???' => 213,
        '???' => 214,
        '???' => 215,
        '???' => 216,
        '???' => 217,
        '???' => 218,
        '???' => 219,
        '???' => 220,
        '???' => 221,
        '???' => 222,
        '???' => 223,

        '??' => 224,
        '??' => 225,
        '??' => 226,
        '??' => 227,
        '??' => 228,
        '??' => 229,
        '??' => 230,
        '??' => 231,
        '??' => 232,
        '??' => 233,
        '??' => 234,
        '??' => 235,
        '???' => 236,
        '??' => 237,
        '??' => 238,
        '???' => 239,

        '???' => 240,
        '??' => 241,
        '???' => 242,
        '???' => 243,
        '???' => 244,
        '???' => 245,
        '??' => 246,
        '???' => 247,
        '??' => 248,
        '???' => 249,
        '??' => 250,
        '???' => 251,
        '???' => 252,
        '??' => 253,
        '???' => 254,

        _ => 0,
    }
}
