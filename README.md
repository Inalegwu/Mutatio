# Mutatio
### *Use your favourite themes from any editor*
*Mutatio(latin) : change*

Mutatio is a converter for theme files across editors, designed to reduce the workload of translating theme files across editors,terminals,etc.

#### *Written in Rust btw* 



## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

Feature requests are welcome, but primarily pull requests for other editors and destinations are welcomed with open arms,As I'm only one person.

Please bear in mind that all features being requested will have to undergo a vetting process and will have to be agreed to meet the overall goals of the project to be accepted.

1. Fork the Project

2. Create your Feature Branch (```git checkout -b me/Amazing Editor```)

3. Commit your Changes (``git commit -m 'Add some Amazing Editor'```)

4. Push to the Branch (```git push origin me/Amazing Editor```)

5. Open a Pull Request


### Project Structure
- cmd : handles command line arguments
- [destination/current] : any folder bearing the name of an editor/terminal holds code related to encoding and decoding the theme.All code and modules here will be marked ```pub``` so that another module can use the logic to write it's own theme in that format.


### Licensce
This project uses the MIT Licensce , See LICENSCE.txt for more
