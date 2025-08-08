load io;
load types;
@@ комент
@# 
    многострочный комент
#@
class test = (type: OOP::!CLASS) {
    type CPU = [] ==> ! {
        reg1(!Integer) => 0;
        pc(!HEX) => 0 as !HEX();
    }
}

alias main = () ==> !Boolean {
    use class text;
    use class {
        types::Integer,
        types::Boolean,
        types::HEX
        types::Logic[]
    } as *;
    immutable define a(!Integer) = 5;
    immutable define b(!Integer) = 5;
    special c(Logic[op: +, args: 2]) = a, b;
    forever {    
        text::display "[c]" {
            type: formated;
        }
    }
    return !Boolean.true

}

@#
!Boolean - тип как bool в C(как у нас !HEX(unsinged char), !Logic[](матем. операции))
use class импортирует из include-файла класс
special для Logic[] обязателен
immutable это как const
forever это как while(1) в С
(line 21) as * как в пайтоне from {что то} import *
#@

