# rtf2text
rtf2text converts rtf files into plaintext.  It supports only minimal rtf
features and relies heavily on backwards compatibility hints that most complex
rtf writers emit.

It has been tested with Cocoa, WordML, and OpenOffice RTF files with a high
degree of success.

# Areas for improvement
Asian languages are not current supported properly.  It probably also doesn't
handle Right-To-Left languages or marks properly either.  European languages
seem to work fine, though.

Support for rtf "destinations" and for tracking control word states is poor,
and consequently display tables, lists, and some other document structure
formatting features aren't well supported.

# Possible new features
With improved rtf destination support and control word state tracking, we could
emit markdown, simplified rtf, or other structured text formats (including
HTML).

Why would you want to parse rtf and emit RTF?  To simplify the document
formatting, reducing overall filesize, or strip out undesired advanced features
(like embedded shapes or images).
