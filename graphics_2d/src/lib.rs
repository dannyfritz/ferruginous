pub trait Transform {}

pub trait Style {}

pub trait Shapes {
    type Scalar;
    type UserError;
    fn circle(&mut self, radius: Self::Scalar, transform: &Transform, style: &Style) -> Result<(), Self::UserError>;
    fn rectangle(&mut self, width: Self::Scalar, height: Self::Scalar, transform: &Transform, style: &Style) -> Result<(), Self::UserError>;
    fn line(&mut self, width: Self::Scalar, transform: &Transform, style: &Style) -> Result<(), Self::UserError>;
}

pub trait ImageData {}

pub trait Image {
    type UserError;
    fn draw(&mut self, imageData: &ImageData, tranform: &Transform) -> Result<(), Self::UserError>;
}

pub trait FontData {}

pub trait Font {
    type UserError;
    fn draw(&mut self, text: String, fontData: &FontData, transform: &Transform, style: &Style) -> Result<(), Self::UserError>;
}

