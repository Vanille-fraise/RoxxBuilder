# RoxxBuilder

Hi, welcome to RoxxBuilder, a 100% open source project developped by myself [@Vanille-fraise](https://github.com/Vanille-fraise/).  
RoxxBuilder is first tool to generate Dofus buid, that optimize the damage of a given spell.  
This tool is a totally free to use API.  
To use it contact me via [Discord](https://discordapp.com/users/480041757862789131) to get the needed informations :)  

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
  POST http://{replace_with_ip:port}/api/evaluate-attack
```
To get the IP address contact me on Discord. See Author section.

Other endpoints:
```http
  GET http://{replace_with_ip:port}/api/breeds         # list of breeds
  GET http://{replace_with_ip:port}/api/attacks        # list of spell variants with their spell levels
  GET http://{replace_with_ip:port}/api/roxx-builder   # health check
```

| Request parameters | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `damages` | `[*Damage_line*]` | The non-crit lines of your attack. |
| `critDamages` | `[*Damage_line*]` | The crit lines of your attack. |
| `damageSource` | `string` | Value: "**Sort**" or "Arme" |
| `damagePosition` | `string` | Value: "**Distance**" or "Melee" |
| `piege` | `boolean` | Value: true or **false** |
| `canCrit` | `boolean` | Value: **true** or false |
| `criticalHitProbability` | `integer` | Base crit in percent. Ex: **0**, 5, 15... |
| `damageCalculation` | `string` | Search calculation type. Value: "Minimized", "Min", "**Average**", "Max".|

Notes: The bold are the default values. A field with default value is optional.

| **Damage_line:** | Type | Description |
| :-------- | :------- | :------------------------- |
| `damageElement` | `integer` | Element id: **0** = Neutre, **1** = Terre, **2** = Feu, **3** = Eau, **4** = Air.|
| `minValue` | `integer` | Lower bound of the damage line. |
| `maxValue` | `integer` | Upper bound of the damage line. |

## API - Usage/Examples
#### 1. Colère de IOP
Request:
```http
  POST http://{replace_with_ip:port}/api/evaluate-attack
```
Body:
```json
  {
    "damages": [
    {"damageElement": 1, "minValue" : 81, "maxValue" : 100}
    ],
    "critDamages": [
    {"damageElement": 1, "minValue" : 97, "maxValue" : 120}
    ],
    "damageSource": "Sort",
    "damagePosition": "Melee",
    "piege": false,
    "canCrit": true,
    "criticalHitProbability": 25,
    "damageCalculation": "Average"
}
```
Answer (trimmed: `items` contains the full item details per slot):
```json
{
    "status": "success",
    "data": {
        "items_name": ["Collier de Tourthon", "Bottes de Brouce", "Paravoile de Mama Ayuto", "Noblesse de Jahash Jurgen", "Jugement de Thanatena", "Surpuissant Chacha de Combat (MJ)", "Ceinture Volkorne", "Coiffe du Comte Harebourg", "Annobusé de Maître Jarbo", "", "Vigoureux majeur", "Forcené majeur", "Enragé", "Enragé majeur", "Furibond majeur", "Robuste majeur"],
        "eval": 2546,
        "builds_evaluated": 217344,
        "search_time": {
            "secs": 1,
            "nanos": 1727083
        },
        "best_build_position": 81919,
        "items": { "...": "per-slot item details (type, names, lvl, img...)" }
    }
}
```
#### 2. Ambuscade
Request:
```http
  POST http://{replace_with_ip:port}/api/evaluate-attack
```
Body:
```json
  {
    "damages": [
    {"damageElement": 3, "minValue" : 10, "maxValue" : 12},
    {"damageElement": 1, "minValue" : 10, "maxValue" : 12},
    {"damageElement": 2, "minValue" : 10, "maxValue" : 12},
    {"damageElement": 4, "minValue" : 10, "maxValue" : 12}
    ],
    "critDamages": [
    {"damageElement": 3, "minValue" : 12, "maxValue" : 14},
    {"damageElement": 1, "minValue" : 12, "maxValue" : 14},
    {"damageElement": 2, "minValue" : 12, "maxValue" : 14},
    {"damageElement": 4, "minValue" : 12, "maxValue" : 14}
    ],
    "damageSource": "Sort",
    "damagePosition": "Distance",
    "piege": false,
    "canCrit": true,
    "criticalHitProbability": 5,
    "damageCalculation": "Average"
}
```



## Author

- [@Vanille-fraise](https://github.com/Vanille-fraise/)
- [Youtube newest video](https://www.youtube.com/watch?v=-dAHVCplywU&ab_channel=SuccessFull)
- [Discord](https://discordapp.com/users/480041757862789131)

