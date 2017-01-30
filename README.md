![Trusty logo](trusty-logo.png)

[![Build Status](https://travis-ci.org/BookOwl/trusty.svg?branch=master)](https://travis-ci.org/BookOwl/trusty)

a mess of code that will (hopefully) one day be a console text editor

## TODO
* Fix unicode issues. Right now the code assumes that each character is exactly 1 byte, which is _wrong_ and will make trusty _panic_ on files that contain multi-byte characters! This is _very bad_!
* Finish the basic editing functionality. Right now you can only move the cursor around and type characters.
* Add syntax highlighting.
* Add find-replace.
* Add auto indention.
* Add the ability to edit multiple files.
* Figure out the best way to let end-users extend trusty. (scripts, maybe?)

## License
I highly doubt that _anyone_ would want to have anything to do with this right now, but if you really want to mess with it, trusty is released under the MIT license.
