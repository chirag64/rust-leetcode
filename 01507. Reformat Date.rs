impl Solution {
    // Split string by space characters. Create an array of dates and months. Then match the split values (split as date, mo)
    pub fn reformat_date(date: String) -> String {
        let split_obj = date.split(" ");
        let parsed_date = split_obj.collect::<Vec<&str>>();
        
        let dates = ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th", "13th", "14th", "15th", "16th", "17th", "18th", "19th", "20th", "11st", "22nd", "23rd", "24th", "25th", "26th", "27th", "28th", "29th", "30th", "31st"];
        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        
        let dd_num = dates.iter().position(|&d| d == parsed_date[0]).unwrap() + 1;
        let dd = if dd_num < 10 {
            format!("0{}", dd_num)
        } else {
            dd_num.to_string()
        };
        
        let mm_num = months.iter().position(|&m| m == parsed_date[1]).unwrap() + 1;
        let mm = if mm_num < 10 {
            format!("0{}", mm_num)
        } else {
            mm_num.to_string()
        };
        
        let yyyy = parsed_date[2];
        
        let output_date = format!("{}-{}-{}", yyyy, mm, dd);
        return output_date;
    }
}
