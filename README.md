# PDF Viewer 
It allows user to upload the pdf and show extracted text in the CLI.

Steps to reproduce the project
1. ```git clone https://github.com/Daksh-10/PDFViewer.git```
2. ```cargo build```
3. ```cargo run```

Dependencies
1. It uses **lopdf** package and **fs** package.
2. lopdf - It allows to parse data from pdf.
3. fs - File system package allows various operations on files ,like read, write, delete, copy, etc.

This has various applications like Resume ATS score calculation, parse over pdfs to provide data to LLMs.
