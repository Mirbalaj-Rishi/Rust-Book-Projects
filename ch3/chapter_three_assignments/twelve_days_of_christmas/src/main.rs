fn main() {
    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    let gifts = ["A partridge in a pear tree","Two turtle doves,","Three French hens,","four calling birds,","five gold rings,","six geese a-laying,","seven swans a-swimming,","eight maids a-milking,","nine ladies dancing,","ten lords a-leaping,","eleven pipers piping,","twelve drummers drumming,"];
    for i in 0..12{
        let day_name = days[i];
        println!("On the {day_name} day of Christmas my true love sent to me");
        for j in (0..i).rev(){
            let gift = gifts[j];
            println!("{gift}");
        }

        println!(" ");

    }


}
