# RoxxBuilder

Hi, welcome to RoxxBuilder, a 100% open source project developped by myself [@Vanille-fraise](https://github.com/Vanille-fraise/).
The first tool to generate Dofus buid, that optimize the damage of a given spell.


## State of the of app

The algorith will be improved gith a graph search.
It should improve the quality of the result build.

#### Incoming features:
- Save of already computed attack to improve the reponse speed.
- Paralelisation of the request (currently every request are queued and treated one by one).
- White list: Items that must figures on the final build.
- Black list: Forbidden items.
- Conditions: The final build must contains the following statistic conditions, such as: 11PA & 6PM; at least 30% res all. The final build will then be the build that do the most damage while passing the conditions.

#### Current Bugs:
- Missing a ring in the final build.

## API - Documentation

#### Retrive a build

```http
  POST http://{replace_with_ip:port}/roxx-builder/attack
```
To get the IP address contact me on Discord. See Author section.
| Request parameters | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `damages` | `[*Damage_line*]` | The non-crit lines of your attack. |
| `crit_damages` | `[*Damage_line*]` | The crit lines of your attack. |
| `damage_source` | `string` | Value: "**Sort**" or "Arme" |
| `damage_position` | `string` | Value: "**Distance**" or "Melee" |
| `piege` | `boolean` | Value: true or **false** |
| `can_crit` | `boolean` | Value: **true** or false |
| `base_crit` | `integer` | Base crit in percent. Ex: **0**, 5 or 15. |
| `damage_calculation` | `string` | Search calculation type. Value: "Minimized", "Min", "**Average**", "Max".|

Notes: The bold are the default values. A field with default value is optional.

| **Damage_line:** | Type | Description |
| :-------- | :------- | :------------------------- |
| `damage_element` | `string` | Value: "DamageEau", "DamageFeu", "DamageTerre", "DamageAir".|
| `min_value` | `integer` | Lower bound of the damage line. |
| `max_value` | `integer` | Upper bound of the damage line. |

## API - Usage/Examples
#### 1. Colère de IOP
Request:
```http
  POST http://{replace_with_ip:port}/roxx-builder/attack
```
Body:
```json
  {
    "damages": [
    {"damage_element": "DamageTerre", "min_value" : 81, "max_value" : 100},
    ],
    "crit_damages": [
    {"damage_element": "DamageTerre", "min_value" : 97, "max_value" : 120},
    ],
    "damage_source": "Sort",
    "damage_position": "Melee",
    "piege": false,
    "can_crit": true,
    "base_crit": 25,
    "damage_calculation": "Average"
}
```
#### 2. Ambuscade
Request:
```http
  POST http://{replace_with_ip:port}/roxx-builder/attack
```
Body:
```json
{
    "damages": [
    {"damage_element": "DamageEau", "min_value" : 10, "max_value" : 12},
    {"damage_element": "DamageTerre", "min_value" : 10, "max_value" : 12},
    {"damage_element": "DamageFeu", "min_value" : 10, "max_value" : 12},
    {"damage_element": "DamageAir", "min_value" : 10, "max_value" : 12}
    ],
    "crit_damages": [
    {"damage_element": "DamageEau", "min_value" : 12, "max_value" : 14},
    {"damage_element": "DamageTerre", "min_value" : 12, "max_value" : 14},
    {"damage_element": "DamageFeu", "min_value" : 12, "max_value" : 14},
    {"damage_element": "DamageAir", "min_value" : 12, "max_value" : 14}
    ],
    "damage_source": "Sort",
    "damage_position": "Distance",
    "piege": false,
    "can_crit": true,
    "base_crit": 5,
    "damage_calculation": "Average"
}
```



## Author

- [@Vanille-fraise](https://github.com/Vanille-fraise/)
- [Youtube newest video](https://www.youtube.com/watch?v=-dAHVCplywU&ab_channel=SuccessFull)
- [Discord](discordapp.com/users/pyjamas_sacre)

