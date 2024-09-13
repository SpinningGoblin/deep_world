# Combat

## Turn-Based

The game will be a turn-based game when it comes to combat, or other "struggle" sections. In initiative order, the players will take their turn, and then the enemy will decide their action when its their turn. I will have to have some kind of manager to keep track of who's turn it is.

## Actions

### Action Options

#### One action

Each character will be able to do 1 action, a move and maybe a few free/bonus actions.

Some characters, as they level up, might be able to do more actions, or they might be able to have more actions given to them by other characters.

#### Action Points

Each ability could take up a number of action points to use, including movement and other maneuver abilities.

## Enemies in Shadow

Some enemies will start _In Shadow_ when the combat begins in a room. Enemies that are still in shadow by the time their turn comes will make their attacks with [advantage](#advantage). Certain creatures/ancestries/classes might be able to put themselves back into shadow.

Enemies will be considered to be in shadow or not in shadow for every member of the party, same for members of the party, where when one is in shadow, they'll be in shadow for all enemies.

There will also be skills for removing creatures from being in shadow, magical light spells, or if some characters can see in the dark, they can let the rest of the party know, maybe for a bonus action.

For reference, the start of this mechanic is based on a mechanic in

-   [Warhammer Quest: Adventure Card Game](https://boardgamegeek.com/boardgame/181521/warhammer-quest-adventure-card-game)

## Exhausted Skills

When players play the game, I want them to have reasons to use different skills, beyond just having some skills be more useful in certain contexts. I also don't necessarily want them to have to feel like they have to go rest to get their skills back all the time, where they only have the most options at the beginning right after a rest.

One way to accomplish this might be to have skills that only become available after a couple of encounters/victories, or after the character starts taking some damage or gets tired.

They also should have some skills/abilities that can be used at any time, as that way there is always something fun to do, even if it's isn't the most exciting ability.

## Grid-based

Most rooms will be divided into two grids. One grid will be for the "main" floor, and then another for an "upper". The upper grid would be for flying characters, or characters on upper parts of the floor. For the most part, the player characters will be on one side of the main grid and the enemies will be on the other.

Characters in the upper grid would be able to attack characters in the lower normally, unless they have specific weapons or skills, which might give them [advantage](#advantage). Characters in the lower grid would at best be able to attack normally with ranged weapons, [disadvantage](#disadvantage) with normal weapons.

There should be ways for characters/creatures to drag others from the upper grid to the lower, using things like nets or similar.

**Caution** - This might be hard to do in a first-person game.

## Advantage and Disadvantage

It would be nice to be able to have some characters get advantage or disadvantage when doing certain actions in combat. Advantage will add to the roll which might change their result.

### Advantage

The roll happens twice, and the best result is taken. Or possibly you roll N + 1 dice, and keep the top N.

### Disadvantage

The roll happens twice, and the worse result is taken. Or possibly you roll N + 1 dice, and keep the bottom N.

## Initiative

Initiative will be decided at the beginning of the fight. It will be a basic roll that is bolstered by their Agility, Dexterity, or something similar. There should also be ways of having characters change their placement in the initiative order, or possibly even completely resetting initiative order.

## Attacking

**Note:** Some ideas taken from the MCDM RPG as well as others.

### Attack Roll

When attacking, [The Roll](./the_roll.md) is used, and the result is compared against a small lookup table which gives the damage and other effects. I think this amount of randomness could be fun, with modifiers able to make higher damage more guaranteed if players have the resources.

### Auto Hit

There is no missing an attack, it will automatically hit, as will enemy attacks. To account for some of this, the HP will likely be upped for both players and monsters.

### Targets, Range and Area of Effect

All attack skills will have a Range and either a number of Targets, or an Area of Effect (AoE).

-   Range - A value from 1 to N, where the value is the number of grid squares away the character must be.
-   Targets - The number of enemies the skill can hit at once. If the value is greater than 1, then the player will be able to click on each enemy to add to the selection.
-   Area of Effect - The shape of the attack as it overlays on the grid, could be a square of 4 cells, a line, or something else.
    -   Will this work if I want to do first person?
