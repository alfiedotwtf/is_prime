# is_prime

Fast arbitrary length prime number checker using the Miller-Rabin primality test algorithm

This module implements the [Miller-Rabin primality
test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
algoritm. Given an arbitrary length integer specified within a string, apply
the probabilistic algorithm to check if the integer may be prime.

The documentation for this crate can be [found
here](https://docs.rs/is_prime/).

# Examples

    extern crate is_prime;
    use is_prime::*;

    fn main() {
      //
      // Without witnesses...
      //

      // The first RSA Prime
      assert!(is_prime("37975227936943673922808872755445627854565536638199") == true);

      // The first RSA Prime + 1
      assert!(is_prime("37975227936943673922808872755445627854565536638200") == false);

      //
      // With witnesses...
      //

      // The first RSA Prime
      assert!(is_prime_with_witnesses("37975227936943673922808872755445627854565536638199", 5) == true);

      // The first RSA Prime + 1
      assert!(is_prime_with_witnesses("37975227936943673922808872755445627854565536638200", 10) == false);
    }

# Support

Please report any bugs or feature requests at:

* [https://gitlab.com/alfiedotwtf/is_prime/issues](https://gitlab.com/alfiedotwtf/is_prime/issues)

Feel free to fork the repository and submit pull requests :)

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Perpetual Copyright (C) by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License and GNU Free Documentation License
as published by the Free Software Foundation, either version 3 of the GPL or
1.3 of the GFDL, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
