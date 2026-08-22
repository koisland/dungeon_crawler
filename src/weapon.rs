// TODO: Is a struct per weapon better. Then impl a trait like Fireable or Swingable.
// Would then need to store in Player Struct

use std::str::FromStr;

use eyre::bail;

pub enum MeleeWeapon {
    Fist,
    GreatClub,
}

impl FromStr for MeleeWeapon {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fist" => MeleeWeapon::Fist,
            "great_club" => MeleeWeapon::GreatClub,
            _ => bail!("Invalid melee weapon: {s}"),
        })
    }
}

pub enum ProjectileWeapon {
    CrystalStaff,
}

impl FromStr for ProjectileWeapon {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "crystal_staff" => ProjectileWeapon::CrystalStaff,
            _ => bail!("Invalid projectile weapon: {s}"),
        })
    }
}

pub enum Weapon {
    Melee(MeleeWeapon),
    Projectile(ProjectileWeapon),
}

impl Weapon {}
