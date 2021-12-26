/// A trait implemented by microtypes
///
/// Provides some useful common functions for working with microtypes
pub trait Microtype {
    /// The type of the wrapped value
    /// For example, the inner type of an `EmailAddress` could be a `String`
    type Inner;

    /// Create a microtype from the inner value
    fn new(inner: Self::Inner) -> Self;

    /// Consume this microtype and return the value it contains
    fn into_inner(self) -> Self::Inner;

    /// Get a shared reference to the inner value
    fn inner(&self) -> &Self::Inner;

    /// Get a mutable reference to the inner value
    fn inner_mut(&mut self) -> &mut Self::Inner;

    /// Explicitly convert from one microtype to another.
    /// This exists as an alternative to `From`/`Into` implementations to make conversions explicit
    fn transmute<T: Microtype<Inner = Self::Inner>>(self) -> T;
}


/// A trait implemented by secret microtypes
///
/// Due to their nature, secret microtypes are more restrictive than regular microtypes:
///  - `inner`, `inner_mut` and `into_inner` are removed, since they can allow accidental use of
///  the contained secret.
///  - `SecretMicrotype` requires `ExposeSecret<Self::Inner>`; to use the contained data, use
///  `.expose_secret()`
///
///  The wrapped type must also implement [`secrecy::Zeroize`]
pub trait SecretMicrotype: secrecy::ExposeSecret<Self::Inner> {
    /// The type of the wrapped value
    /// For example, the inner type of a `Password` could be a `String`
    type Inner: secrecy::Zeroize;

    /// Create a microtype from the inner value
    /// 
    /// Note that it is not possible to retrieve the owned value, it can only be read via shared
    /// reference obtained via `expose_secret()`
    fn new(inner: Self::Inner) -> Self;
}

pub use secrecy;
