# WIP: A tool to compress your PDF(s) locally with the magic of WebAssembly

## What is this

A private PDF compression site powered by [WebAssembly][webassembly]. Most PDF compression site out there is storing your PDF in their server, some claims they encrypt the file stored in their server. Since most of hardware nowadays can easily do PDF compression locally, this tool took another approach: that is levering your device extra power to compress your PDF files locally, which enabled by the WebAssembly technology. Main caveat of using this method is it may not run in low-end devices or big PDF file, since the compressed PDF file will took the available RAM and will take a while if it's runned in limited CPU. It will still be better to use cloud compression site to compress your PDF if you are running a lower-end devices, but for the most use cases you can use this tool.

This project is released under the [MIT license][license], feel free to use and/or study the source code with attribution to the copyright owner.

## How the compression work

PDF itself is a document format that embeds compressed files (images, fonts, vector graphics, etc), most common compressed in zlib or LZW algorithm. This compression may vary depending on how user is specifying the compression level, resulting in bigger size on faster compression method. To ensure every embedded files in PDF is using the smallest compression method. This tool first have to decompress each files and then compress it once again with the best possible compression method.

Most compression done to reduce the overall filesize of the PDF file is by compressing the images, most commonly formatted in JPEG format. JPEG is a [lossy][lossy-compression] in the sense that reducing the file size means to reduce the image quality itself (by reducing number of colors) which resulted in degredation and information loss. The default setting provided in this tool should be minimizing the filesize while still preserving the quality of images in the PDF.

## Before-after comparison

PDF compression that have been tested using this method:

| PDF file | Before Compressed | After Compressed |
| -------- | ----------------- | ---------------- |
|          |                   |                  |
|          |                   |                  |
|          |                   |                  |
|          |                   |                  |

## Known limitations

-   Haven't been tested in LZW-compressed PDF
-   Will not work in 16-bit and 32-bit pixel images
-   Other image formats is currently not lossy-compressed

If you encounter any of these limitations in problematic way(s). Kindly raise an issue and/or send a pull request to this repository.

## I like this work, what can I do

As simple as it may seems, this project takes time to shape as it is right now: to provide a useful tool and helpful educational resources as well as the extra efforts put into maintaining the source code and repository. This has been a labor of love for myself. Here something you can do:

-   Star this repository on GitHub
-   Fork this project and make your own version
-   Contribute to the source code by raising an issue and/or send a pull request
-   Donate when possible (if I set a donation button in GitHub)
-   Checkout dependencies that used in this project, they may also need some help
-   Spread kindness 💝

[webassembly]: https://en.wikipedia.org/wiki/WebAssembly
[license]: ./LICENSE
[lossy-compression]: https://en.wikipedia.org/wiki/Lossy_compression
