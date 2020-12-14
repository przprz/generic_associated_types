#![feature(generic_associated_types)]

mod gat_on_nightly {
    /// https://www.reddit.com/r/rust/comments/k4vzvp/gats_on_nightly/
    /// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=61caef82814783feadc33a3b865fe8b3

    trait Monad /* : Applicative (for pure/return, doesn't matter for this example) */ {
        // Self is like the "f a" in haskell

        /// extract the "a" from "f a"
        type Unplug;

        /// exchange the "a" in "f a" in the type of Self with B
        type Plug<T>: Monad;

        fn bind<T, F>(self, f: F) -> Self::Plug<T>
            where
                F: Fn(Self::Unplug) -> Self::Plug<T>;
    }

    impl<A> Monad for Option<A> {
        type Unplug = A;
        type Plug<T> = Option<T>;
        fn bind<T, F>(self, f: F) -> Option<T>
            where
                F: Fn(A) -> Option<T>,
        {
            self.and_then(f)
        }
    }

    #[test]
    fn main() {
        assert_eq!(Some(2), Some(1).bind(|x| Some(x * 2)), "it works");
    }
}

mod fpcomplete {
    /// https://www.fpcomplete.com/blog/monads-gats-nightly-rust

    mod _monofunctor_ {
        #[derive(Debug, PartialEq)]
        enum MyOption<A> {
            Some(A),
            None,
        }

        impl<A> MyOption<A> {
            fn map<F: FnOnce(A) -> B, B>(self, f: F) -> MyOption<B> {
                match self {
                    MyOption::Some(a) => MyOption::Some(f(a)),
                    MyOption::None => MyOption::None,
                }
            }
        }

        #[test]
        fn test_option_map() {
            assert_eq!(MyOption::Some(5).map(|x| x + 1), MyOption::Some(6));
            assert_eq!(MyOption::None.map(|x: i32| x + 1), MyOption::None);
        }

        #[derive(Debug, PartialEq)]
        enum MyResult<A, E> {
            Ok(A),
            Err(E),
        }

        impl<A, E> MyResult<A, E> {
            fn map<F: FnOnce(A) -> B, B>(self, f: F) -> MyResult<B, E> {
                match self {
                    MyResult::Ok(a) => MyResult::Ok(f(a)),
                    MyResult::Err(e) => MyResult::Err(e),
                }
            }
        }

        #[test]
        fn test_result_map() {
            assert_eq!(MyResult::Ok(5).map(|x| x + 1), MyResult::Ok::<i32, ()>(6));
        }
    }

    mod functor {
        trait Functor {
            type Unwrapped;
            type Wrapped<B>: Functor;

            fn map<F, B>(self, f: F) -> Self::Wrapped<B>
                where
                    F: FnMut(Self::Unwrapped) -> B;
        }

        impl<A> Functor for Option<A> {
            type Unwrapped = A;
            /// B in `Wrapped<B>` can be of different type than Unwrapped
            type Wrapped<B> = Option<B>;

            fn map<F: FnMut(A) -> B, B>(self, mut f: F) -> Option<B> {
                match self {
                    Some(x) => Some(f(x)),
                    None => None,
                }
            }
        }

        fn ping(_a: i32) -> String {
            /// B in `Wrapped<B>` can be of different type than Unwrapped
            "fixed".into()
        }

        #[test]
        fn test_option_map() {
            assert_eq!(Some(5).map(|x| ping(x)), Some(String::from("fixed")));
            assert_eq!(None.map(|x: i32| x + 1), None);
        }
    }
}

fn main() {
    println!("it works");
}