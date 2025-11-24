/*
 * The contents of this file are subject to the Mozilla Public License Version
 * 2.0 (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 * https://www.mozilla.org/en-US/MPL/2.0/
 *
 * Software distributed under the License is distributed on an "AS IS" basis,
 * WITHOUT WARRANTY OF ANY KIND, either express or implied. See the License
 * for the specific language governing rights and limitations under the
 * License.
 *
 * The Original Code is Libvoikko: Library of natural language processing tools.
 * The Initial Developer of the Original Code is Harri Pitkänen <hatapitk@iki.fi>.
 * Portions created by the Initial Developer are Copyright (C) 2012
 * the Initial Developer. All Rights Reserved.
 *
 * Raudikko, the Java port of the Initial Code is Copyright (C) 2020 by
 * Evident Solutions Oy. All Rights Reserved.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * either the GNU General Public License Version 2 or later (the "GPL"), or
 * the GNU Lesser General Public License Version 2.1 or later (the "LGPL"),
 * in which case the provisions of the GPL or the LGPL are applicable instead
 * of those above. If you wish to allow use of your version of this file only
 * under the terms of either the GPL or the LGPL, and not to allow others to
 * use your version of this file under the terms of the MPL, indicate your
 * decision by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL or the LGPL. If you do not delete
 * the provisions above, a recipient may use your version of this file under
 * the terms of any one of the MPL, the GPL or the LGPL.
 */

use std::io::{BufRead, BufReader, Read};

pub(crate) struct MyInputStream<R: Read> {
    stream: BufReader<R>,
    position: usize,
}

impl<R: Read> MyInputStream<R> {
    pub(crate) fn new(stream: R) -> Self {
        MyInputStream {
            stream: BufReader::new(stream),
            position: 0,
        }
    }

    pub(crate) fn read_byte(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        self.stream.read_exact(&mut buf)?;
        self.position += 1;
        Ok(buf[0])
    }

    pub(crate) fn read_short(&mut self) -> std::io::Result<i16> {
        let b1 = self.read_byte()? as i16;
        let b2 = self.read_byte()? as i16;
        Ok((b2 << 8) | b1)
    }

    pub(crate) fn read_int(&mut self) -> std::io::Result<i32> {
        let b1 = self.read_byte()? as i32;
        let b2 = self.read_byte()? as i32;
        let b3 = self.read_byte()? as i32;
        let b4 = self.read_byte()? as i32;
        Ok((b4 << 24) | (b3 << 16) | (b2 << 8) | b1)
    }

    pub(crate) fn read_int24(&mut self) -> std::io::Result<i32> {
        let b1 = self.read_byte()? as i32;
        let b2 = self.read_byte()? as i32;
        let b3 = self.read_byte()? as i32;
        Ok((b3 << 16) | (b2 << 8) | b1)
    }

    pub(crate) fn read_utf8_string(&mut self) -> std::io::Result<String> {
        let mut bytes = Vec::new();
        loop {
            let b = self.read_byte()?;
            if b == 0 {
                break;
            }
            bytes.push(b);
        }
        String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub(crate) fn has_more(&mut self) -> std::io::Result<bool> {
        let buf = self.stream.fill_buf()?;
        Ok(!buf.is_empty())
    }

    pub(crate) fn skip_n_bytes(&mut self, n: usize) -> std::io::Result<()> {
        for _ in 0..n {
            self.read_byte()?;
        }
        Ok(())
    }

    pub(crate) fn get_position(&self) -> usize {
        self.position
    }
}
