use rand::seq::SliceRandom;
use serenity::async_trait;

use crate::domain::message::Food;
pub struct FoodRepository;

static FOOD_LIST: [&str; 39] = [
    "台式泡菜",
    "起司馬鈴薯",
    "屎",
    "早餐 義大利肉醬麵+蛋+起司 濃湯 大冰奶 薯餅",
    "紫米飯糰",
    "水餃",
    "櫻花蝦炒飯",
    "泡麵",
    "鍋燒意麵",
    "豬排飯",
    "牛排",
    "咖哩飯",
    "擔擔麵",
    "義大利麵",
    "義大利燉飯",
    "羊肉飯",
    "餛飩麵",
    "牛肉麵",
    "燒臘",
    "燒肉飯",
    "雞肉飯",
    "牛肉丼飯",
    "雞排飯",
    "滷味",
    "鍋貼",
    "焗烤飯",
    "蛋包飯",
    "蒸餃",
    "小火鍋",
    "滷味",
    "丹丹漢堡",
    "金拱門",
    "KFC",
    "漢堡王",
    "蔥抓餅",
    "蔥油餅",
    "紅豆餅",
    "雞腿Bang當",
    "酸辣湯餃",
];

#[async_trait]
impl Food for FoodRepository {
    async fn get_food(num: usize) -> Vec<String> {
        let mut rng = &mut rand::thread_rng();
        let result: Vec<String> = FOOD_LIST
            .choose_multiple(&mut rng, num)
            .cloned()
            .map(|x| x.to_string())
            .collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_one_food() {
        let food = FoodRepository::get_food(1).await;

        assert_eq!(1, food.len());
        assert!(FOOD_LIST.contains(&food[0].as_str()));
    }
    #[tokio::test]
    async fn get_multiple_food() {
        let foods = FoodRepository::get_food(3).await;

        assert_eq!(3, foods.len());
        for each in foods {
            assert!(FOOD_LIST.contains(&each.as_str()));
        }
    }
}
