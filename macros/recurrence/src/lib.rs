#[macro_export]
macro_rules! recurrence {
    ( $a:ident[$i:ident]: $t:ty = $($v:expr),+; ...; $e:expr ) => {
        {
            struct Recurrence {
                values: Vec<$t>,
                n: usize,
            }

            impl Iterator for Recurrence {
                type Item = $t;

                fn next(&mut self) -> Option<Self::Item> {
                    let $i = self.n;
                    let $a = &mut self.values;
                    if $i == $a.len() {
                        $a.push($e);
                    }
                    self.n += 1;
                    Some($a[$i])
                }
            }

            Recurrence { values: vec![$($v, )*], n: 0 }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        let fib = recurrence![a[n]: i32 = 0, 1; ...; a[n-1] + a[n-2]];
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34],
            fib.take(10).collect::<Vec<i32>>()
        );
    }

    #[test]
    fn test_factorial() {
        let factorial = recurrence![f[i]: f64 = 1.0; ...; f[i-1] * i as f64];
        assert_eq!(
            vec![1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0],
            factorial.take(10).collect::<Vec<f64>>()
        );
    }

    #[test]
    fn test_non_literal() {
        let powers_of_2 = recurrence![a[i]: i32 = 1 + 1; ...; a[i-1] * 2];
        assert_eq!(
            vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            powers_of_2.take(10).collect::<Vec<i32>>()
        );
    }
}
