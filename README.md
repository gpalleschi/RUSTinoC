# RUSTinoC
A simple RUST utility to convert value in differents formats.  

Use: RUSTinoC \<Type Conv\> \<Value To Conv\>  

\<Type Conv\>     : Conversion Type to apply, values permitted are :  

    -hd : Hex to Decimal   
    -dh : Decimal to Hex   
    -bd : Binary to Decimal   
    -db : Decimal to Binary   
    -od : Octal to Decimal   
    -do : Decimal to Octal   
  
\<Value To Conv\> : Value to Convert
  
Examples :  

    1) RUSTinoC -hd a0ef  

                Hex 'a0ef' ---> Dec '-24337'  

    2) RUSTinoC -dh 250   

                Dec '250' ---> Hex 'fa'  

### Prerequisites  

None  

## Built With  

* [Visual Code Editor](https://code.visualstudio.com)   
* rustc version rustc 1.65.0 (897e37553 2022-11-02)  

## Authors  

* **Giovanni Palleschi** - [gpalleschi](https://github.com/gpalleschi)  

## License

This project is licensed under the GNU GENERAL PUBLIC LICENSE 3.0 License - see the [LICENSE](LICENSE) file for details 