/*******************************************************************************
 * The MIT License (MIT)
 *
 * Copyright (c) 2024, Jean-David Gadina - www.xs-labs.com
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the Software), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED AS IS, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 ******************************************************************************/

pub mod xs_io
{
    enum SeekDirection
    {
        Current,
        Begin,
        End,
    }

    enum Endianness
    {
        Default,
        LittleEndian,
        BigEndian,
    }

    trait BinaryStream
    {
        fn preferred_endianness(     &self ) -> Endianness;
        fn set_preferred_endianness( &mut self, endianness: Endianness );
        
        fn tell( &self ) -> u64;
        fn seek( &mut self, offset: i64, direction: SeekDirection ) -> Result< (), Box< dyn std::error::Error > >;
        
        fn read(     &mut self, size: u64 ) -> Result< Vec< u8 >, Box< dyn std::error::Error > >;
        fn read_all( &mut self )            -> Result< Vec< u8 >, Box< dyn std::error::Error > >;
        
        fn has_bytes_available( &mut self ) -> bool
        {
            match self.available_bytes()
            {
                Ok(  _ ) => return true,
                Err( _ ) => return false,
            }
        }

        fn available_bytes( &mut self ) -> Result< u64, Box< dyn std::error::Error > >;
        
        fn read_u8( &mut self ) -> Result< u8, Box< dyn std::error::Error > >
        {
            match self.read( 1 )
            {
                Ok( value )  => return Ok( value[ 0 ] ),
                Err( error ) => return Err( error ),
            }
        }
        
        fn read_i8( &mut self ) -> Result< i8, Box< dyn std::error::Error > >
        {
            match self.read_u8()
            {
                Ok( value )  => return Ok( value as i8 ),
                Err( error ) => return Err( error ),
            }
        }
        
        fn read_u16(               &mut self ) -> Result< u16, Box< dyn std::error::Error > >;
        fn read_big_endian_u16(    &mut self ) -> Result< u16, Box< dyn std::error::Error > >;
        fn read_little_endian_u16( &mut self ) -> Result< u16, Box< dyn std::error::Error > >;
        
        fn read_u32(               &mut self ) -> Result< u32, Box< dyn std::error::Error > >;
        fn read_big_endian_u32(    &mut self ) -> Result< u32, Box< dyn std::error::Error > >;
        fn read_little_endian_u32( &mut self ) -> Result< u32, Box< dyn std::error::Error > >;
        
        fn read_u64(               &mut self ) -> Result< u64, Box< dyn std::error::Error > >;
        fn read_big_endian_u64(    &mut self ) -> Result< u64, Box< dyn std::error::Error > >;
        fn read_little_endian_u64( &mut self ) -> Result< u64, Box< dyn std::error::Error > >;
        
        fn read_big_endian_fixed_point(    &mut self, integer_length: u32, fractional_length: u32 ) -> Result< f64, Box< dyn std::error::Error > >;
        fn read_little_endian_fixed_point( &mut self, integer_length: u32, fractional_length: u32 ) -> Result< f64, Box< dyn std::error::Error > >;
        
        fn read_pascal_string(                &mut self )              -> Result< String, Box< dyn std::error::Error > >;
        fn read_string(                       &mut self, length: u64 ) -> Result< String, Box< dyn std::error::Error > >;
        fn read_null_terminated_string(       &mut self )              -> Result< String, Box< dyn std::error::Error > >;
        fn read_null_terminated_utf16_string( &mut self )              -> Result< String, Box< dyn std::error::Error > >;
    }
}
