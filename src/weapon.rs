// TODO: Is a struct per weapon better. Then impl a trait like Fireable or Swingable.
// Would then need to store in Player Struct

use std::str::FromStr;

use eyre::bail;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
pub enum MeleeWeapon {
    Fist,
    GreatClub,
}

impl FromStr for MeleeWeapon {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Fist" => MeleeWeapon::Fist,
            "GreatClub" => MeleeWeapon::GreatClub,
            _ => bail!("Invalid melee weapon: {s}"),
        })
    }
}

#[derive(IntoStaticStr)]
pub enum ProjectileWeapon {
    CrystalStaff,
}

impl FromStr for ProjectileWeapon {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CrystalStaff" => ProjectileWeapon::CrystalStaff,
            _ => bail!("Invalid projectile weapon: {s}"),
        })
    }
}

pub enum Weapon {
    Melee(MeleeWeapon),
    Projectile(ProjectileWeapon),
}

impl Weapon {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Weapon::Melee(melee_weapon) => melee_weapon.into(),
            Weapon::Projectile(projectile_weapon) => projectile_weapon.into(),
        }
    }
}
