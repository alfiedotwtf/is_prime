# is_prime

Fast arbitrary length prime number checker using the Miller-Rabin primality test algorithm

This module implements the [Miller-Rabin primality
test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test). Given
an arbitrary length number specified within a string, apply the probabilistic
algorithm to check if the number is composite. If the number is not composite,
it may be prime.

The documentation for this crate can be [found
here](https://docs.rs/is_prime/).

# Examples

    extern crate is_prime;
    use is_prime::*;

    fn main() {
      for i in 0..42 {
        println!("Is {} prime? {}", i, is_prime(i.to_string().as_str()));
      }
    }

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/is_prime/issues](https://github.com/alfiedotwtf/is_prime/issues)

Watch the repository and keep up with the latest changes:

* [https://github.com/alfiedotwtf/is_prime/subscription](https://github.com/alfiedotwtf/is_prime/subscription)

Feel free to fork the repository and submit pull requests :)

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Copyright (C) 2017 by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License and GNU Free Documentation License
as published by the Free Software Foundation, either version 3 of the GPL or
1.3 of the GFDL, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
