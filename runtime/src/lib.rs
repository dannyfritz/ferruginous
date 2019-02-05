pub trait Runtime {
    type Context;
    type UserData;
    type UserError;
    type Scalar;
    update(dt: Self::Scalar, context: &Self::Context, userData: &Self::UserData) -> Result<(), Self::UserError>;
    render(context: &Self::Context, userData: &Self::UserData) -> Result<(), Self::UserError>;
}
