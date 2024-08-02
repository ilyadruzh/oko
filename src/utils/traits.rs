// Написать функцию, принимающую слайс generic объектов и выводящую на экран все элементы этого слайса. Продемонстрировать, что данная функция работает с любыми типами, реализующими необходимый трейт.
// Для вывода объектов на экран можно использовать трейт Printable из примеров выше. Данный трейт потребуется реализовать на типах, которые будут использоваться при демонстрации.
// Можешь приложить ссылку на код по желанию

trait Printable {
    fn print(&self) {
        println!("float: {}", self)
    }

    fn some_function() {
        todo!()
    }
}

fn print_object<T: Printable + ?Sized>(t: &T) {
    T::some_function(); // Ошибка! Откуда компилятору взять адрес данной функции?
    t.print()
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self)
    }
}

// Для impl блоков тоже можно указывать generic параметры.
// Тут мы реализуем ToString для ссылки на слайс,
// в котором могут быть элементы любых типов.
impl<T> Printable for &[T] {
    fn print(&self) {
        println!("slice with {} elements", self.len())
    }
}

fn print_object<T: Printable>(item: T) {
    // Метод print доступен, так как в функцию можно передать
    // только объекты, для которых реализован trait Printable.
    item.print()
}

fn print_slice<T>(array: [T]) {
    for item in array {
        println!("{}", item);
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    print_object(&array[..]);

    let number = 42.0;
    let dynamic_object: &dyn Printable = &number;
    print_object(dynamic_object) // используем `&dyn Printable` в качестве `T: Printable`.
    
}
