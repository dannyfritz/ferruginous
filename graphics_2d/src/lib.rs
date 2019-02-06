pub trait Transform {}
pub trait Extra {}

pub trait Shapes<T: Transform, E: Extra> {
    type Scalar;
    type UserError;
    fn circle(
        &mut self,
        radius: Self::Scalar,
        transform: &T,
        extra: &E,
    ) -> Result<(), Self::UserError>;
    fn rectangle(
        &mut self,
        width: Self::Scalar,
        height: Self::Scalar,
        transform: &T,
        extra: &E,
    ) -> Result<(), Self::UserError>;
    fn line(
        &mut self,
        width: Self::Scalar,
        transform: &T,
        extra: &E,
    ) -> Result<(), Self::UserError>;
}

pub trait ImageData {}

pub trait Image<T: Transform, E: Extra> {
    type UserError;
    fn draw(
        &mut self,
        image_data: &ImageData,
        tranform: &T,
        style: &E,
    ) -> Result<(), Self::UserError>;
}

pub trait FontData {}

pub trait Font<T: Transform, E: Extra, F: FontData> {
    type UserError;
    fn draw(
        &mut self,
        text: String,
        font_data: &F,
        transform: &T,
        extra: &E,
    ) -> Result<(), Self::UserError>;
}
