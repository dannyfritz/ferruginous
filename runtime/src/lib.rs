pub trait Runtime {
    type Context;
    type UserData;
    type UserError;
    type Scalar;
    fn update(dt: Self::Scalar, context: &Self::Context, userData: &Self::UserData) -> Result<(), Self::UserError>;
    fn render(context: &Self::Context, userData: &Self::UserData) -> Result<(), Self::UserError>;
}
