#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxDrawing {
    // Simple Lines
    Horizontal,
    Vertical,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    VerticalRight,
    VerticalLeft,
    HorizontalDown,
    HorizontalUp,
    Cross,
    
    // Double Lines
    DoubleHorizontal,
    DoubleVertical,
    DoubleDownRight,
    DoubleDownLeft,
    DoubleUpRight,
    DoubleUpLeft,
    DoubleVerticalRight,
    DoubleVerticalLeft,
    DoubleHorizontalDown,
    DoubleHorizontalUp,
    DoubleCross,
    
    // Mixed Lines (Single-Doble)
    HorizontalDoubleDown,
    HorizontalDoubleUp,
    HorizontalDoubleVertical,
    VerticalDoubleRight,
    VerticalDoubleLeft,
    VerticalDoubleHorizontal,
    DownDoubleRight,
    DownDoubleLeft,
    DownDoubleHorizontal,
    UpDoubleRight,
    UpDoubleLeft,
    UpDoubleHorizontal,
    DoubleDownSingleRight,
    DoubleDownSingleLeft,
    DoubleUpSingleRight,
    DoubleUpSingleLeft,
    DoubleVerticalSingleRight,
    DoubleVerticalSingleLeft,
    SingleHorizontalDoubleDown,
    SingleHorizontalDoubleUp,
    SingleVerticalDoubleRight,
    SingleVerticalDoubleLeft,
    
    // Special Corners and Connections
    ArcDownRight,
    ArcDownLeft,
    ArcUpRight,
    ArcUpLeft,
    DiagonalUpperLeftToLowerRight,
    DiagonalUpperRightToLowerLeft,
}

impl BoxDrawing {
    pub fn as_char(&self) -> char {
        match self {
            // Simple Lines
            Self::Horizontal => '─',
            Self::Vertical => '│',
            Self::DownRight => '┌',
            Self::DownLeft => '┐',
            Self::UpRight => '└',
            Self::UpLeft => '┘',
            Self::VerticalRight => '├',
            Self::VerticalLeft => '┤',
            Self::HorizontalDown => '┬',
            Self::HorizontalUp => '┴',
            Self::Cross => '┼',
            
            // Double Lines
            Self::DoubleHorizontal => '═',
            Self::DoubleVertical => '║',
            Self::DoubleDownRight => '╔',
            Self::DoubleDownLeft => '╗',
            Self::DoubleUpRight => '╚',
            Self::DoubleUpLeft => '╝',
            Self::DoubleVerticalRight => '╠',
            Self::DoubleVerticalLeft => '╣',
            Self::DoubleHorizontalDown => '╦',
            Self::DoubleHorizontalUp => '╩',
            Self::DoubleCross => '╬',
            
            // Mixed Lines (Single-Doble)
            Self::HorizontalDoubleDown => '╤',
            Self::HorizontalDoubleUp => '╧',
            Self::HorizontalDoubleVertical => '╪',
            Self::VerticalDoubleRight => '╟',
            Self::VerticalDoubleLeft => '╢',
            Self::VerticalDoubleHorizontal => '╫',
            Self::DownDoubleRight => '╓',
            Self::DownDoubleLeft => '╖',
            Self::DownDoubleHorizontal => '╥',
            Self::UpDoubleRight => '╙',
            Self::UpDoubleLeft => '╜',
            Self::UpDoubleHorizontal => '╨',
            Self::DoubleDownSingleRight => '╒',
            Self::DoubleDownSingleLeft => '╕',
            Self::DoubleUpSingleRight => '╘',
            Self::DoubleUpSingleLeft => '╛',
            Self::DoubleVerticalSingleRight => '╞',
            Self::DoubleVerticalSingleLeft => '╡',
            Self::SingleHorizontalDoubleDown => '╤',
            Self::SingleHorizontalDoubleUp => '╧',
            Self::SingleVerticalDoubleRight => '╟',
            Self::SingleVerticalDoubleLeft => '╢',

            // Special Corners and Connections
            Self::ArcDownRight => '╭',
            Self::ArcDownLeft => '╮',
            Self::ArcUpRight => '╰',
            Self::ArcUpLeft => '╯',
            Self::DiagonalUpperLeftToLowerRight => '╱',
            Self::DiagonalUpperRightToLowerLeft => '╲',
        }
    }
}

// Thanks chatgpt for the help with the BoxDrawing enum