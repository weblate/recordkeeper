open = Open
save = Save
undo = Undo
redo = Redo
download = Save & Download
ok = OK

# The project's info page
nav_about = About
# The project's source code
nav_source = Source
# The project's software license
nav_license = License

-arts = Arts
-skills = Skills
-class = Class

# Objects without a registered name.
# $id: the ID of the object
unnamed = #{ $id } (No Name)

# Data related to the base game (no DLC)
menu_category_base = Base Game Data

# Data related to DLCs 1-3
menu_category_dlc = DLC Data

# Data related to Future Redeemed (DLC 4)
menu_category_dlc4 = Future Redeemed Data

# Data related to the save file itself
menu_category_meta = Metadata

# Dangerous settings that should not be changed by the average user
menu_category_danger = Danger Zone

menu_base_characters = Characters
menu_base_ouroboros = Ouroboros
menu_base_items = Items
menu_base_field = Field
menu_base_quests = Quests
menu_base_ums = Unique Monsters
menu_base_formations = Party Formations

menu_dlc_powaugment = Inoswap
menu_dlc_challenge = Challenge Battle
menu_dlc_gauntlet = Archsage's Gauntlet
menu_dlc_masha = Accessory Crafting

menu_dlc4_growth = Affinity Growth
menu_dlc4_collepedia = Collectopedia
menu_dlc4_enemypedia = Enemypedia

menu_meta_meta = Save Info

menu_danger_flags = Flags

## Save info translations

hours = Hours
minutes = Minutes
seconds = Seconds
date = Date
time = Time

difficulty = Difficulty
difficulty_easy = Easy
difficulty_normal = Normal
difficulty_hard = Hard
difficulty_veryhard = Very Hard (Unreleased)

scenario_flag_flag = Flag
scenario_flag_chapter = Chapter
# $id: the chapter ID
scenario_flag_chapter_id = Chapter { $id }

meta_playtime = Play Time
# When the game was last saved
meta_savetime = Save Time
meta_scenario_flag = Scenario Flag
meta_ngp = New Game Plus
meta_settings = Settings

## Ouroboros translations

ouroboros_sp = SP
ouroboros_linked_skills = Linked { -skills }
ouroboros_skill_tree = Skill Tree

## Flag screen

# The flag ID
flag_index = Flag Index
# How many bits the flag uses
flag_bits = Flag Bits
# The value stored in the flag
flag_value = Value
# Placeholder text for the flag ID input field.
# Allows the user to jump to the page that contains the flag
flag_jump_page = Go to flag...

## Items screen

item_type = Item Type
item_slot_index = Slot ID
item_item = Item
item_amount = Amount
item_actions = Actions

# You are searching item slots for items
item_search = Search item slots...
item_first_empty = Go to empty slot

# Item types

item_type_collection = Collectibles
item_type_accessory = Accessories
item_type_precious = Key Items
item_type_gem = Gems
item_type_cylinder = Cylinders
item_masha = Crafted Accessory
item_unnamed = (No Name)

# Item rarities

item_rarity_common = Common
item_rarity_rare = Rare
item_rarity_legendary = Legendary

# Accessory crafting

masha_item_type = Accessory Name
masha_level = Craft Level
masha_enhance = Effect
masha_boost_stat = Statistic
masha_boost_value = Value

masha_stat_hp = HP
masha_stat_str = Strength
masha_stat_heal = Healing Power
masha_stat_dex = Dexterity
masha_stat_agi = Agility
masha_stat_crit = Crit Rate
masha_stat_block = Block Rate

## Quest screen

quest_id = ID
quest_name = Name
quest_status = Status
quest_actions = Actions

quest_status_unstarted = Not Started
quest_status_progress = In Progress
quest_status_complete_a = Completed (A)
quest_status_complete_b = Completed (B)

quest_purpose = Edit Quest Objectives
quest_purpose_id = Objective ID
quest_purpose_status = Objective Status
quest_purpose_tasks_a = Tasks (Branch A)
quest_purpose_tasks_b = Tasks (Branch B)

quest_task_ask = Ask
quest_task_battle = Battle
quest_task_chase = Chase
quest_task_collect = Collect
quest_task_collepedia = Collectopedia
quest_task_condition = Condition
quest_task_event = Event
quest_task_follow = Follow
quest_task_gimmick = Gimmick
quest_task_reach = Location
quest_task_request = Item List
quest_task_talk = Talk


## Character screen

character_character = Character
character_class = Edit Class
character_level = Level
character_exp = EXP
character_bexp = Bonus EXP
character_selected_class = Selected {-class}

# The character's selected costume
character_costume = Costume

# The level the character first joined the party at
character_arrival_level = Initial Level

# Character clothes dirty level
character_dirt = Dirtiness

# $id: the art slot ID (0-6). 
# 0 = talent art
# 1-3 = Keves arts
# 4-6 = Agnus arts
character_art = 
    { $id ->
        [0] Talent Art
        [4] Agnus Art 1
        [5] Agnus Art 2
        [6] Agnus Art 3
       *[other] Keves Art { $id }
    }
# $id: the slot ID
character_skill = 
    { $id ->
        [0] Class Skill 1
        [1] Class Skill 2
        [2] Class Skill 3
        [3] Class Skill 4
        [4] Inherited Skill 1
        [5] Inherited Skill 2
        [6] Inherited Skill 3
        *[7] Unused Skill
    }
# $id: the slot ID
character_gem =
    { $id ->
        [0] Gem Type 1
        [1] Gem Type 2
        *[2] Gem Type 3
    }

character_class_cp = CP
character_class_unlock = Unlock Points
character_class_rank = Rank