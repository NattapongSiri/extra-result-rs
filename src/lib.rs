#![no_std]

/// Add extra functionality to the [Result] type.
/// This trait provides a set of async versions of the standard [Result] methods.
/// Unlike the standard methods, these methods accept async functions as arguments.
/// It return a [Future] that resolves to the same result of standard [Result] counterpart.
/// Unless async functions is needed, it is recommended to use the standard [Result] methods for performance reason.
pub trait ExtraResult<T, E> {
    /// Same as [Result::map] but took async functions.
    /// 
    /// It calls the async function with the value inside the [Result] if it is Ok.
    /// If the [Result] is Err, it returns the error.
    fn map_fut<U, F>(self, f: F) -> impl Future<Output = Result<U, E>>
    where
        F: AsyncFnOnce(T) -> U;
    /// Same as [Result::map_or] but took async functions.
    /// 
    /// It calls the async function with the value inside the [Result] if it is Ok.
    /// If the [Result] is Err, it returns the default value passed in.
    fn map_or_fut<U, F>(self, default: U, f: F) -> impl Future<Output = U>
    where
        F: AsyncFnOnce(T) -> U;
    /// Same as [Result::map_or_else] but took async functions.
    /// 
    /// It took two functions arguments, one for the Ok case and one for the Err case.
    /// The `default` function is called with the error value if the [Result] is Err.
    /// The `f` function is called with the value inside the [Result] if it is Ok.
    /// The return type of the `default` function must be the same as the return type of the `f` function.
    fn map_or_else_fut<U, D, F>(self, default: D, f: F) -> impl Future<Output = U>
    where
        D: AsyncFnOnce(E) -> U,
        F: AsyncFnOnce(T) -> U;
    /// Same as [Result::map_err] but took async functions.
    /// 
    /// It calls the async function with the error value inside the [Result] if it is Err.
    /// If the [Result] is Ok, it return the same result as original.
    fn map_err_fut<F, U>(self, f: F) -> impl Future<Output = Result<T, U>>
    where
        F: AsyncFnOnce(E) -> U;
    /// Same as [Result::inspect] but took async functions.
    /// 
    /// It calls the async function with the value inside the [Result] if it is Ok.
    /// If the [Result] is Err, it won't call the function.
    /// The function have no effect on the result of the [Result].
    fn inspect_fut<F>(self, f: F) -> impl Future<Output = Self>
    where
        F: AsyncFnOnce(&T);
    /// Same as [Result::inspect_err] but took async functions.
    ///
    /// It calls the async function with the error value inside the [Result] if it is Err.
    /// If the [Result] is Ok, it return the same result as original.
    /// The function have no effect on the result of the [Result].
    fn inspect_err_fut<F>(self, f: F) -> impl Future<Output = Self>
    where
        F: AsyncFnOnce(&E);
    /// Same as [Result::and_then] but took async functions.
    /// 
    /// It calls the async function with the value inside the [Result] if it is Ok.
    /// If the [Result] is Err, it return the same result as original.
    fn and_then_fut<U, F>(self, f: F) -> impl Future<Output = Result<U, E>>
    where
        F: AsyncFnOnce(T) -> Result<U, E>;
    /// Same as [Result::or_else] but took async functions.
    ///
    /// It calls the async function with the error value inside the [Result] if it is Err.
    /// If the [Result] is Ok, it return the same result as original.
    fn or_else_fut<U, F>(self, f: F) -> impl Future<Output = Result<T, U>>
    where
        F: AsyncFnOnce(E) -> Result<T, U>;
    /// Same as [Result::unwrap_or_else] but took async functions.
    /// 
    /// It calls the async function with the error value inside the [Result] if it is Err.
    /// If the [Result] is Ok, it return the same result as original.
    /// The function must return the same type as the [Result].
    fn unwrap_or_else_fut<F>(self, f: F) -> impl Future<Output = T>
    where
        F: AsyncFnOnce(E) -> T;
    /// Check if the [Result] is Ok and apply the async function to it.
    /// This is a mirror implementation of [Result::is_ok_and] but for async functions.
    /// 
    /// It calls the async function with the value inside the [Result] if it is Ok.
    /// If the [Result] is Err, it return false.
    /// The function must return a boolean value.
    fn is_ok_and_fut<F>(self, f: F) -> impl Future<Output = bool>
    where
        F: AsyncFnOnce(&T) -> bool;
    /// Check if the [Result] is Err and apply the async function to it.
    /// This is a mirror implementation of [Result::is_err_and] but for async functions.
    ///
    /// It calls the async function with the error value inside the [Result] if it is Err.
    /// If the [Result] is Ok, it return false.
    /// The function must return a boolean value.
    fn is_err_and_fut<F>(self, f: F) -> impl Future<Output = bool>
    where
        F: AsyncFnOnce(&E) -> bool;
}

impl<T, E> ExtraResult<T, E> for Result<T, E> {
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::map] but for async functions.
    #[inline]
    fn map_fut<U, F>(self, f: F) -> impl Future<Output = Result<U, E>>
    where
        Self: Sized,
        F: AsyncFnOnce(T) -> U,
    {
        async {
            match self {
                Ok(v) => Ok(f(v).await),
                Err(e) => Err(e),
            }
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::map_or] but for async functions.
    #[inline]
    fn map_or_fut<U, F>(self, default: U, f: F) -> impl Future<Output = U>
    where
        Self: Sized,
        F: AsyncFnOnce(T) -> U,
    {
        async {
            match self {
                Ok(v) => f(v).await,
                Err(_) => default,
            }
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::map_or_else] but for async functions.
    #[inline]
    fn map_or_else_fut<U, D, F>(self, default: D, f: F) -> impl Future<Output = U>
    where
        Self: Sized,
        D: AsyncFnOnce(E) -> U,
        F: AsyncFnOnce(T) -> U,
    {
        async {
            match self {
                Ok(v) => f(v).await,
                Err(e) => default(e).await,
            }
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::map_err] but for async functions.
    #[inline]
    fn map_err_fut<F, U>(self, f: F) -> impl Future<Output = Result<T, U>>
    where
        F: AsyncFnOnce(E) -> U,
    {
        async {
            match self {
                Ok(v) => Ok(v),
                Err(e) => Err(f(e).await),
            }
        }
    }
    /// Inspect the value of a [Result] with async function.
    /// This is a mirror implementation of [Result::inspect] but for async functions.
    #[inline]
    fn inspect_fut<F>(self, f: F) -> impl Future<Output = Self>
    where
        F: AsyncFnOnce(&T),
    {
        async move {
            if let Ok(ref v) = self {
                f(v).await;
            }
            self
        }
    }
    /// Inspect the error of a [Result] with async function.
    /// This is a mirror implementation of [Result::inspect_err] but for async functions.
    #[inline]
    fn inspect_err_fut<F>(self, f: F) -> impl Future<Output = Self>
    where
        F: AsyncFnOnce(&E),
    {
        async move {
            if let Err(ref e) = self {
                f(e).await;
            }
            self
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::and_then] but for async functions.
    #[inline]
    fn and_then_fut<U, F>(self, f: F) -> impl Future<Output = Result<U, E>>
    where
        Self: Sized,
        F: AsyncFnOnce(T) -> Result<U, E>,
    {
        async {
            match self {
                Ok(v) => f(v).await,
                Err(e) => Err(e),
            }
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::or_else] but for async functions.
    #[inline]
    fn or_else_fut<U, F>(self, f: F) -> impl Future<Output = Result<T, U>>
    where
        F: AsyncFnOnce(E) -> Result<T, U>,
    {
        async {
            match self {
                Ok(v) => Ok(v),
                Err(e) => f(e).await,
            }
        }
    }
    /// Convert a [Result] into another [Result] with async mapping function.
    /// This is a mirror implementation of [Result::unwrap_or_else] but for async functions.
    #[inline]
    fn unwrap_or_else_fut<F>(self, f: F) -> impl Future<Output = T>
    where
        F: AsyncFnOnce(E) -> T,
    {
        async {
            match self {
                Ok(v) => v,
                Err(e) => f(e).await,
            }
        }
    }
    /// Check if the [Result] is Ok and apply the async function to it.
    /// This is a mirror implementation of [Result::is_ok_and] but for async functions.
    #[inline]
    fn is_ok_and_fut<F>(self, f: F) -> impl Future<Output = bool>
    where
        F: AsyncFnOnce(&T) -> bool,
    {
        async {
            match self {
                Ok(v) => f(&v).await,
                Err(_) => false,
            }
        }
    }
    /// Check if the [Result] is Err and apply the async function to it.
    /// This is a mirror implementation of [Result::is_err_and] but for async functions.
    #[inline]
    fn is_err_and_fut<F>(self, f: F) -> impl Future<Output = bool>
    where
        F: AsyncFnOnce(&E) -> bool,
    {
        async {
            match self {
                Ok(_) => false,
                Err(e) => f(&e).await,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn map_on_ok() {
        Result::<u8, ()>::Ok(1)
            .map_fut(async |x| { x + 1 }).await
            .map(|r| assert_eq!(r, 2u8)).unwrap();
    }
    #[tokio::test]
    async fn map_on_err() {
        Result::<u8, u8>::Err(1)
            .map_fut(async |x| { x + 1 }).await
            .map_err(|r| assert_eq!(r, 1u8)).unwrap_err();
    }
    #[tokio::test]
    async fn map_or_on_ok() {
        assert_eq!(
            Result::<u8, ()>::Ok(1)
                .map_or_fut(3, async |x| { x + 1 }).await, 
            2u8
        );
    }
    #[tokio::test]
    async fn map_or_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .map_or_fut(3u8, async |x| { x + 1 }).await, 
            3u8)
        ;
    }
    #[tokio::test]
    async fn map_or_else_on_ok() {
        assert_eq!(
            Result::<u8, u8>::Ok(1)
                .map_or_else_fut(async |x| x - 1, async |x| x + 1 ).await, 
            2u8
        );
    }
    #[tokio::test]
    async fn map_or_else_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .map_or_else_fut(async |x| x - 1, async |x| x + 1 ).await, 
            0u8
        );
    }
    #[tokio::test]
    async fn map_err_on_ok() {
        Result::<u8, u8>::Ok(1)
            .map_err_fut(async |x| { x + 1 }).await
            .map(|r| assert_eq!(r, 1u8)).unwrap();
    }
    #[tokio::test]
    async fn map_err_on_err() {
        Result::<u8, u8>::Err(1)
            .map_err_fut(async |x| { x + 1 }).await
            .map_err(|r| assert_eq!(r, 2u8)).unwrap_err();
    }
    #[tokio::test]
    async fn inspect_on_ok() {
        let mut val = 0;
        Result::<u8, ()>::Ok(1)
            .inspect_fut(async |_| { val = 1; }).await
            .map(|r| assert_eq!(r, 1u8)).unwrap();
        assert_eq!(val, 1);
    }
    #[tokio::test]
    async fn inspect_on_err() {
        Result::<u8, u8>::Err(1)
            .inspect_fut(async |_| { panic!("This should never be called") }).await
            .map_err(|r| assert_eq!(r, 1u8)).unwrap_err();
    }
    #[tokio::test]
    async fn inspect_err_on_ok() {
        Result::<u8, u8>::Ok(1)
            .inspect_err_fut(async |_| { panic!("This should never be called") }).await
            .map(|r| assert_eq!(r, 1u8)).unwrap();
    }
    #[tokio::test]
    async fn inspect_err_on_err() {
        let mut val = 0;
        Result::<u8, u8>::Err(1)
            .inspect_err_fut(async |_| { val = 1; }).await
            .map_err(|r| assert_eq!(r, 1u8)).unwrap_err();
        assert_eq!(val, 1);
    }
    #[tokio::test]
    async fn and_then_on_ok() {
        Result::<u8, ()>::Ok(1)
            .and_then_fut(async |x| { Ok(x + 1) }).await
            .map(|r| assert_eq!(r, 2u8)).unwrap();
    }
    #[tokio::test]
    async fn and_then_on_err() {
        Result::<u8, u8>::Err(1)
            .and_then_fut(async |x| { Ok(x + 1) }).await
            .map_err(|r| assert_eq!(r, 1u8)).unwrap_err();
    }
    #[tokio::test]
    async fn or_else_ok_on_ok() {
        Result::<u8, u8>::Ok(1)
            .or_else_fut::<u8, _>(async |x| { Ok(x + 1) }).await
            .map(|r| assert_eq!(r, 1u8)).unwrap();
    }
    #[tokio::test]
    async fn or_else_err_on_ok() {
        Result::<u8, u8>::Ok(1)
            .or_else_fut(async |x| { Err(x + 1) }).await // This should never be called
            .map(|r| assert_eq!(r, 1u8)).unwrap();
    }
    #[tokio::test]
    async fn or_else_ok_on_err() {
        Result::<u8, u8>::Err(1)
            .or_else_fut::<u8, _>(async |x| { Ok(x + 1) }).await // Like a recovered error.
            .map(|r| assert_eq!(r, 2u8)).unwrap();
    }
    #[tokio::test]
    async fn or_else_err_on_err() {
        Result::<u8, u8>::Err(1)
            .or_else_fut(async |x| { Err(x + 1) }).await
            .map_err(|r| assert_eq!(r, 2u8)).unwrap_err();
    }
    #[tokio::test]
    async fn unwrap_or_else_on_ok() {
        assert_eq!(
            Result::<u8, u8>::Ok(1)
                .unwrap_or_else_fut(async |x| { x + 1 }).await, 
            1u8
        );
    }
    #[tokio::test]
    async fn unwrap_or_else_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .unwrap_or_else_fut(async |x| { x + 1 }).await, 
            2u8
        );
    }
    #[tokio::test]
    async fn is_ok_true_and_on_ok() {
        assert_eq!(
            Result::<u8, ()>::Ok(1)
                .is_ok_and_fut(async |x| { *x == 1 }).await, 
            true
        );
    }
    #[tokio::test]
    async fn is_ok_false_and_on_ok() {
        assert_eq!(
            Result::<u8, ()>::Ok(1)
                .is_ok_and_fut(async |x| { *x != 1 }).await, 
            false
        );
    }
    #[tokio::test]
    async fn is_ok_true_and_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .is_ok_and_fut(async |x| { *x == 1 }).await, 
            false
        );
    }
    #[tokio::test]
    async fn is_ok_false_and_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .is_ok_and_fut(async |x| { *x != 1 }).await, 
            false
        );
    }
    #[tokio::test]
    async fn is_err_true_and_on_ok() {
        assert_eq!(
            Result::<u8, u8>::Ok(1)
                .is_err_and_fut(async |x| { *x == 1 }).await, 
            false
        );
    }
    #[tokio::test]
    async fn is_err_false_and_on_ok() {
        assert_eq!(
            Result::<u8, u8>::Ok(1)
                .is_err_and_fut(async |x| { *x != 1 }).await, 
            false
        );
    }
    #[tokio::test]
    async fn is_err_true_and_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .is_err_and_fut(async |x| { *x == 1 }).await, 
            true
        );
    }
    #[tokio::test]
    async fn is_err_false_and_on_err() {
        assert_eq!(
            Result::<u8, u8>::Err(1)
                .is_err_and_fut(async |x| { *x != 1 }).await, 
            false
        );
    }
}
