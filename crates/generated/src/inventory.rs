// This file is @generated. Please do not edit.

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Area {
    Storage,
    CraftingOutput,
    CraftingInput,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Hotbar,
    Offhand,
    FurnaceIngredient,
    FurnaceFuel,
    FurnaceOutput,
    EnchantmentItem,
    EnchantmentLapis,
    BrewingBottle,
    BrewingIngredient,
    BrewingBlazePowder,
    VillagerInput,
    VillagerOutput,
    BeaconPayment,
    AnvilInput1,
    AnvilInput2,
    AnvilOutput,
    Saddle,
    HorseArmor,
    LlamaCarpet,
    CartographyMap,
    CartographyPaper,
    CartographyOutput,
    GrindstoneInput1,
    GrindstoneInput2,
    GrindstoneOutput,
    LecternBook,
    LoomBanner,
    LoomDye,
    LoomPattern,
    LoomOutput,
    StonecutterInput,
    StonecutterOutput,
}
#[derive(Debug, Clone)]
pub enum Window {
    Player {
        player: crate::Inventory,
    },
    Generic9x1 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9x2 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9x3 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9x4 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9x5 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9x6 {
        left_chest: crate::Inventory,
        right_chest: crate::Inventory,
        player: crate::Inventory,
    },
    Generic3x3 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Crafting {
        crafting_table: crate::Inventory,
        player: crate::Inventory,
    },
    Furnace {
        furnace: crate::Inventory,
        player: crate::Inventory,
    },
    BlastFurnace {
        blast_furnace: crate::Inventory,
        player: crate::Inventory,
    },
    Smoker {
        smoker: crate::Inventory,
        player: crate::Inventory,
    },
    Enchantment {
        enchantment_table: crate::Inventory,
        player: crate::Inventory,
    },
    BrewingStand {
        brewing_stand: crate::Inventory,
        player: crate::Inventory,
    },
    Beacon {
        beacon: crate::Inventory,
        player: crate::Inventory,
    },
    Anvil {
        anvil: crate::Inventory,
        player: crate::Inventory,
    },
    Hopper {
        hopper: crate::Inventory,
        player: crate::Inventory,
    },
    ShulkerBox {
        shulker_box: crate::Inventory,
        player: crate::Inventory,
    },
    Cartography {
        cartography_table: crate::Inventory,
        player: crate::Inventory,
    },
    Grindstone {
        grindstone: crate::Inventory,
        player: crate::Inventory,
    },
    Lectern {
        lectern: crate::Inventory,
        player: crate::Inventory,
    },
    Loom {
        loom: crate::Inventory,
        player: crate::Inventory,
    },
    Stonecutter {
        stonecutter: crate::Inventory,
        player: crate::Inventory,
    },
}
impl Window {
    #[allow(unused_comparisons)]
    pub fn index_to_slot(&self, index: usize) -> Option<(&crate::Inventory, Area, usize)> {
        match self {
            Window::Player { player } => {
                if index >= 0 && index < 1 {
                    let area = Area::CraftingOutput;
                    let slot = index - 0;
                    Some((player, area, slot))
                } else if index >= 1 && index < 5 {
                    let area = Area::CraftingInput;
                    let slot = index - 1;
                    Some((player, area, slot))
                } else if index >= 5 && index < 6 {
                    let area = Area::Helmet;
                    let slot = index - 5;
                    Some((player, area, slot))
                } else if index >= 6 && index < 7 {
                    let area = Area::Chestplate;
                    let slot = index - 6;
                    Some((player, area, slot))
                } else if index >= 7 && index < 8 {
                    let area = Area::Leggings;
                    let slot = index - 7;
                    Some((player, area, slot))
                } else if index >= 8 && index < 9 {
                    let area = Area::Boots;
                    let slot = index - 8;
                    Some((player, area, slot))
                } else if index >= 9 && index < 36 {
                    let area = Area::Storage;
                    let slot = index - 9;
                    Some((player, area, slot))
                } else if index >= 36 && index < 45 {
                    let area = Area::Hotbar;
                    let slot = index - 36;
                    Some((player, area, slot))
                } else if index >= 45 && index < 46 {
                    let area = Area::Offhand;
                    let slot = index - 45;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x1 { block, player } => {
                if index >= 0 && index < 9 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 9 && index < 36 {
                    let area = Area::Storage;
                    let slot = index - 9;
                    Some((player, area, slot))
                } else if index >= 36 && index < 45 {
                    let area = Area::Hotbar;
                    let slot = index - 36;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x2 { block, player } => {
                if index >= 0 && index < 18 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 18 && index < 45 {
                    let area = Area::Storage;
                    let slot = index - 18;
                    Some((player, area, slot))
                } else if index >= 45 && index < 54 {
                    let area = Area::Hotbar;
                    let slot = index - 45;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x3 { block, player } => {
                if index >= 0 && index < 27 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 27 && index < 54 {
                    let area = Area::Storage;
                    let slot = index - 27;
                    Some((player, area, slot))
                } else if index >= 54 && index < 63 {
                    let area = Area::Hotbar;
                    let slot = index - 54;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x4 { block, player } => {
                if index >= 0 && index < 36 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 36 && index < 63 {
                    let area = Area::Storage;
                    let slot = index - 36;
                    Some((player, area, slot))
                } else if index >= 63 && index < 72 {
                    let area = Area::Hotbar;
                    let slot = index - 63;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x5 { block, player } => {
                if index >= 0 && index < 45 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 45 && index < 72 {
                    let area = Area::Storage;
                    let slot = index - 45;
                    Some((player, area, slot))
                } else if index >= 72 && index < 81 {
                    let area = Area::Hotbar;
                    let slot = index - 72;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic9x6 {
                left_chest,
                right_chest,
                player,
            } => {
                if index >= 0 && index < 27 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((left_chest, area, slot))
                } else if index >= 27 && index < 54 {
                    let area = Area::Storage;
                    let slot = index - 27;
                    Some((right_chest, area, slot))
                } else if index >= 54 && index < 81 {
                    let area = Area::Storage;
                    let slot = index - 54;
                    Some((player, area, slot))
                } else if index >= 81 && index < 90 {
                    let area = Area::Hotbar;
                    let slot = index - 81;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Generic3x3 { block, player } => {
                if index >= 0 && index < 9 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((block, area, slot))
                } else if index >= 9 && index < 36 {
                    let area = Area::Storage;
                    let slot = index - 9;
                    Some((player, area, slot))
                } else if index >= 36 && index < 45 {
                    let area = Area::Hotbar;
                    let slot = index - 36;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Crafting {
                crafting_table,
                player,
            } => {
                if index >= 0 && index < 1 {
                    let area = Area::CraftingOutput;
                    let slot = index - 0;
                    Some((crafting_table, area, slot))
                } else if index >= 1 && index < 10 {
                    let area = Area::CraftingInput;
                    let slot = index - 1;
                    Some((crafting_table, area, slot))
                } else if index >= 10 && index < 37 {
                    let area = Area::Storage;
                    let slot = index - 10;
                    Some((player, area, slot))
                } else if index >= 37 && index < 46 {
                    let area = Area::Hotbar;
                    let slot = index - 37;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Furnace { furnace, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::FurnaceIngredient;
                    let slot = index - 0;
                    Some((furnace, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::FurnaceFuel;
                    let slot = index - 1;
                    Some((furnace, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::FurnaceOutput;
                    let slot = index - 2;
                    Some((furnace, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::BlastFurnace {
                blast_furnace,
                player,
            } => {
                if index >= 0 && index < 1 {
                    let area = Area::FurnaceIngredient;
                    let slot = index - 0;
                    Some((blast_furnace, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::FurnaceFuel;
                    let slot = index - 1;
                    Some((blast_furnace, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::FurnaceOutput;
                    let slot = index - 2;
                    Some((blast_furnace, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Smoker { smoker, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::FurnaceIngredient;
                    let slot = index - 0;
                    Some((smoker, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::FurnaceFuel;
                    let slot = index - 1;
                    Some((smoker, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::FurnaceOutput;
                    let slot = index - 2;
                    Some((smoker, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Enchantment {
                enchantment_table,
                player,
            } => {
                if index >= 0 && index < 1 {
                    let area = Area::EnchantmentItem;
                    let slot = index - 0;
                    Some((enchantment_table, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::EnchantmentLapis;
                    let slot = index - 1;
                    Some((enchantment_table, area, slot))
                } else if index >= 2 && index < 29 {
                    let area = Area::Storage;
                    let slot = index - 2;
                    Some((player, area, slot))
                } else if index >= 29 && index < 38 {
                    let area = Area::Hotbar;
                    let slot = index - 29;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::BrewingStand {
                brewing_stand,
                player,
            } => {
                if index >= 0 && index < 3 {
                    let area = Area::BrewingBottle;
                    let slot = index - 0;
                    Some((brewing_stand, area, slot))
                } else if index >= 3 && index < 4 {
                    let area = Area::BrewingIngredient;
                    let slot = index - 3;
                    Some((brewing_stand, area, slot))
                } else if index >= 4 && index < 5 {
                    let area = Area::BrewingBlazePowder;
                    let slot = index - 4;
                    Some((brewing_stand, area, slot))
                } else if index >= 5 && index < 32 {
                    let area = Area::Storage;
                    let slot = index - 5;
                    Some((player, area, slot))
                } else if index >= 32 && index < 41 {
                    let area = Area::Hotbar;
                    let slot = index - 32;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Beacon { beacon, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::BeaconPayment;
                    let slot = index - 0;
                    Some((beacon, area, slot))
                } else if index >= 1 && index < 28 {
                    let area = Area::Storage;
                    let slot = index - 1;
                    Some((player, area, slot))
                } else if index >= 28 && index < 37 {
                    let area = Area::Hotbar;
                    let slot = index - 28;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Anvil { anvil, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::AnvilInput1;
                    let slot = index - 0;
                    Some((anvil, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::AnvilInput2;
                    let slot = index - 1;
                    Some((anvil, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::AnvilOutput;
                    let slot = index - 2;
                    Some((anvil, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Hopper { hopper, player } => {
                if index >= 0 && index < 4 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((hopper, area, slot))
                } else if index >= 4 && index < 31 {
                    let area = Area::Storage;
                    let slot = index - 4;
                    Some((player, area, slot))
                } else if index >= 31 && index < 40 {
                    let area = Area::Hotbar;
                    let slot = index - 31;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::ShulkerBox {
                shulker_box,
                player,
            } => {
                if index >= 0 && index < 27 {
                    let area = Area::Storage;
                    let slot = index - 0;
                    Some((shulker_box, area, slot))
                } else if index >= 27 && index < 54 {
                    let area = Area::Storage;
                    let slot = index - 27;
                    Some((player, area, slot))
                } else if index >= 54 && index < 63 {
                    let area = Area::Hotbar;
                    let slot = index - 54;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Cartography {
                cartography_table,
                player,
            } => {
                if index >= 0 && index < 1 {
                    let area = Area::CartographyMap;
                    let slot = index - 0;
                    Some((cartography_table, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::CartographyPaper;
                    let slot = index - 1;
                    Some((cartography_table, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::CartographyOutput;
                    let slot = index - 2;
                    Some((cartography_table, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Grindstone { grindstone, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::GrindstoneInput1;
                    let slot = index - 0;
                    Some((grindstone, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::GrindstoneInput2;
                    let slot = index - 1;
                    Some((grindstone, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::GrindstoneOutput;
                    let slot = index - 2;
                    Some((grindstone, area, slot))
                } else if index >= 3 && index < 30 {
                    let area = Area::Storage;
                    let slot = index - 3;
                    Some((player, area, slot))
                } else if index >= 30 && index < 39 {
                    let area = Area::Hotbar;
                    let slot = index - 30;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Lectern { lectern, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::LecternBook;
                    let slot = index - 0;
                    Some((lectern, area, slot))
                } else if index >= 1 && index < 28 {
                    let area = Area::Storage;
                    let slot = index - 1;
                    Some((player, area, slot))
                } else if index >= 28 && index < 37 {
                    let area = Area::Hotbar;
                    let slot = index - 28;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Loom { loom, player } => {
                if index >= 0 && index < 1 {
                    let area = Area::LoomBanner;
                    let slot = index - 0;
                    Some((loom, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::LoomDye;
                    let slot = index - 1;
                    Some((loom, area, slot))
                } else if index >= 2 && index < 3 {
                    let area = Area::LoomPattern;
                    let slot = index - 2;
                    Some((loom, area, slot))
                } else if index >= 3 && index < 4 {
                    let area = Area::LoomOutput;
                    let slot = index - 3;
                    Some((loom, area, slot))
                } else if index >= 4 && index < 31 {
                    let area = Area::Storage;
                    let slot = index - 4;
                    Some((player, area, slot))
                } else if index >= 31 && index < 40 {
                    let area = Area::Hotbar;
                    let slot = index - 31;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
            Window::Stonecutter {
                stonecutter,
                player,
            } => {
                if index >= 0 && index < 1 {
                    let area = Area::StonecutterInput;
                    let slot = index - 0;
                    Some((stonecutter, area, slot))
                } else if index >= 1 && index < 2 {
                    let area = Area::StonecutterOutput;
                    let slot = index - 1;
                    Some((stonecutter, area, slot))
                } else if index >= 2 && index < 29 {
                    let area = Area::Storage;
                    let slot = index - 2;
                    Some((player, area, slot))
                } else if index >= 29 && index < 38 {
                    let area = Area::Hotbar;
                    let slot = index - 29;
                    Some((player, area, slot))
                } else {
                    None
                }
            }
        }
    }
    pub fn slot_to_index(
        &self,
        inventory: &crate::Inventory,
        area: Area,
        slot: usize,
    ) -> Option<usize> {
        match self {
            Window::Player { player } => {
                if area == Area::CraftingOutput && player.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::CraftingInput && player.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Helmet && player.ptr_eq(inventory) {
                    Some(slot + 5)
                } else if area == Area::Chestplate && player.ptr_eq(inventory) {
                    Some(slot + 6)
                } else if area == Area::Leggings && player.ptr_eq(inventory) {
                    Some(slot + 7)
                } else if area == Area::Boots && player.ptr_eq(inventory) {
                    Some(slot + 8)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 9)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 36)
                } else if area == Area::Offhand && player.ptr_eq(inventory) {
                    Some(slot + 45)
                } else {
                    None
                }
            }
            Window::Generic9x1 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 9)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 36)
                } else {
                    None
                }
            }
            Window::Generic9x2 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 18)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 45)
                } else {
                    None
                }
            }
            Window::Generic9x3 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 27)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 54)
                } else {
                    None
                }
            }
            Window::Generic9x4 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 36)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 63)
                } else {
                    None
                }
            }
            Window::Generic9x5 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 45)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 72)
                } else {
                    None
                }
            }
            Window::Generic9x6 {
                left_chest,
                right_chest,
                player,
            } => {
                if area == Area::Storage && left_chest.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && right_chest.ptr_eq(inventory) {
                    Some(slot + 27)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 54)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 81)
                } else {
                    None
                }
            }
            Window::Generic3x3 { block, player } => {
                if area == Area::Storage && block.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 9)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 36)
                } else {
                    None
                }
            }
            Window::Crafting {
                crafting_table,
                player,
            } => {
                if area == Area::CraftingOutput && crafting_table.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::CraftingInput && crafting_table.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 10)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 37)
                } else {
                    None
                }
            }
            Window::Furnace { furnace, player } => {
                if area == Area::FurnaceIngredient && furnace.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::FurnaceFuel && furnace.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::FurnaceOutput && furnace.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::BlastFurnace {
                blast_furnace,
                player,
            } => {
                if area == Area::FurnaceIngredient && blast_furnace.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::FurnaceFuel && blast_furnace.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::FurnaceOutput && blast_furnace.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::Smoker { smoker, player } => {
                if area == Area::FurnaceIngredient && smoker.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::FurnaceFuel && smoker.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::FurnaceOutput && smoker.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::Enchantment {
                enchantment_table,
                player,
            } => {
                if area == Area::EnchantmentItem && enchantment_table.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::EnchantmentLapis && enchantment_table.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 29)
                } else {
                    None
                }
            }
            Window::BrewingStand {
                brewing_stand,
                player,
            } => {
                if area == Area::BrewingBottle && brewing_stand.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::BrewingIngredient && brewing_stand.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::BrewingBlazePowder && brewing_stand.ptr_eq(inventory) {
                    Some(slot + 4)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 5)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 32)
                } else {
                    None
                }
            }
            Window::Beacon { beacon, player } => {
                if area == Area::BeaconPayment && beacon.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 28)
                } else {
                    None
                }
            }
            Window::Anvil { anvil, player } => {
                if area == Area::AnvilInput1 && anvil.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::AnvilInput2 && anvil.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::AnvilOutput && anvil.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::Hopper { hopper, player } => {
                if area == Area::Storage && hopper.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 4)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 31)
                } else {
                    None
                }
            }
            Window::ShulkerBox {
                shulker_box,
                player,
            } => {
                if area == Area::Storage && shulker_box.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 27)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 54)
                } else {
                    None
                }
            }
            Window::Cartography {
                cartography_table,
                player,
            } => {
                if area == Area::CartographyMap && cartography_table.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::CartographyPaper && cartography_table.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::CartographyOutput && cartography_table.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::Grindstone { grindstone, player } => {
                if area == Area::GrindstoneInput1 && grindstone.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::GrindstoneInput2 && grindstone.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::GrindstoneOutput && grindstone.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 30)
                } else {
                    None
                }
            }
            Window::Lectern { lectern, player } => {
                if area == Area::LecternBook && lectern.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 28)
                } else {
                    None
                }
            }
            Window::Loom { loom, player } => {
                if area == Area::LoomBanner && loom.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::LoomDye && loom.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::LoomPattern && loom.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::LoomOutput && loom.ptr_eq(inventory) {
                    Some(slot + 3)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 4)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 31)
                } else {
                    None
                }
            }
            Window::Stonecutter {
                stonecutter,
                player,
            } => {
                if area == Area::StonecutterInput && stonecutter.ptr_eq(inventory) {
                    Some(slot + 0)
                } else if area == Area::StonecutterOutput && stonecutter.ptr_eq(inventory) {
                    Some(slot + 1)
                } else if area == Area::Storage && player.ptr_eq(inventory) {
                    Some(slot + 2)
                } else if area == Area::Hotbar && player.ptr_eq(inventory) {
                    Some(slot + 29)
                } else {
                    None
                }
            }
        }
    }
}
#[allow(warnings)]
#[allow(clippy::all)]
impl Window {
    /// Returns the `name` property of this `Window`.
    pub fn name(&self) -> &'static str {
        match self {
            Window::Player { .. } => "player",
            Window::Generic9x1 { .. } => "generic_9x1",
            Window::Generic9x2 { .. } => "generic_9x2",
            Window::Generic9x3 { .. } => "generic_9x3",
            Window::Generic9x4 { .. } => "generic_9x4",
            Window::Generic9x5 { .. } => "generic_9x5",
            Window::Generic9x6 { .. } => "generic_9x6",
            Window::Generic3x3 { .. } => "generic_3x3",
            Window::Crafting { .. } => "crafting",
            Window::Furnace { .. } => "furnace",
            Window::BlastFurnace { .. } => "blast_furnace",
            Window::Smoker { .. } => "smoker",
            Window::Enchantment { .. } => "enchantment",
            Window::BrewingStand { .. } => "brewing_stand",
            Window::Beacon { .. } => "beacon",
            Window::Anvil { .. } => "anvil",
            Window::Hopper { .. } => "hopper",
            Window::ShulkerBox { .. } => "shulker_box",
            Window::Cartography { .. } => "cartography",
            Window::Grindstone { .. } => "grindstone",
            Window::Lectern { .. } => "lectern",
            Window::Loom { .. } => "loom",
            Window::Stonecutter { .. } => "stonecutter",
        }
    }
}
#[derive(Debug, Clone)]
pub enum InventoryBacking<T> {
    Player {
        crafting_input: [T; 4],
        crafting_output: [T; 1],
        helmet: [T; 1],
        chestplate: [T; 1],
        leggings: [T; 1],
        boots: [T; 1],
        storage: [T; 27],
        hotbar: [T; 9],
    },
    Chest {
        storage: [T; 27],
    },
    CraftingTable {
        crafting_input: [T; 9],
        crafting_output: [T; 1],
    },
    Furnace {
        furnace_ingredient: [T; 1],
        furnace_fuel: [T; 1],
        furnace_output: [T; 1],
    },
}
impl<T> InventoryBacking<T> {
    pub fn area_slice(&self, area: Area) -> Option<&[T]> {
        match self {
            InventoryBacking::Player {
                crafting_input,
                crafting_output,
                helmet,
                chestplate,
                leggings,
                boots,
                storage,
                hotbar,
            } => match area {
                Area::CraftingInput => Some(crafting_input.as_ref()),
                Area::CraftingOutput => Some(crafting_output.as_ref()),
                Area::Helmet => Some(helmet.as_ref()),
                Area::Chestplate => Some(chestplate.as_ref()),
                Area::Leggings => Some(leggings.as_ref()),
                Area::Boots => Some(boots.as_ref()),
                Area::Storage => Some(storage.as_ref()),
                Area::Hotbar => Some(hotbar.as_ref()),
                _ => None,
            },
            InventoryBacking::Chest { storage } => match area {
                Area::Storage => Some(storage.as_ref()),
                _ => None,
            },
            InventoryBacking::CraftingTable {
                crafting_input,
                crafting_output,
            } => match area {
                Area::CraftingInput => Some(crafting_input.as_ref()),
                Area::CraftingOutput => Some(crafting_output.as_ref()),
                _ => None,
            },
            InventoryBacking::Furnace {
                furnace_ingredient,
                furnace_fuel,
                furnace_output,
            } => match area {
                Area::FurnaceIngredient => Some(furnace_ingredient.as_ref()),
                Area::FurnaceFuel => Some(furnace_fuel.as_ref()),
                Area::FurnaceOutput => Some(furnace_output.as_ref()),
                _ => None,
            },
        }
    }
    pub fn player() -> Self
    where
        T: Default,
    {
        InventoryBacking::Player {
            crafting_input: Default::default(),
            crafting_output: Default::default(),
            helmet: Default::default(),
            chestplate: Default::default(),
            leggings: Default::default(),
            boots: Default::default(),
            storage: Default::default(),
            hotbar: Default::default(),
        }
    }
    pub fn chest() -> Self
    where
        T: Default,
    {
        InventoryBacking::Chest {
            storage: Default::default(),
        }
    }
    pub fn crafting_table() -> Self
    where
        T: Default,
    {
        InventoryBacking::CraftingTable {
            crafting_input: Default::default(),
            crafting_output: Default::default(),
        }
    }
    pub fn furnace() -> Self
    where
        T: Default,
    {
        InventoryBacking::Furnace {
            furnace_ingredient: Default::default(),
            furnace_fuel: Default::default(),
            furnace_output: Default::default(),
        }
    }
}
