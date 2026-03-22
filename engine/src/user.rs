use nix::unistd::{User, Uid};

/// Recibe lo que llega de usuario_so del kernel.
/// Si es numérico (UID), lo resuelve a nombre via /etc/passwd.
/// Si ya es un nombre, lo devuelve directo.
/// Si no puede resolver, devuelve el string original sin paniquear.
pub fn resolve_username(raw: &str) -> String {
    match raw.trim().parse::<u32>() {
        Ok(uid) => resolve_uid(uid),
        Err(_) => raw.to_owned(),
    }
}

fn resolve_uid(uid: u32) -> String {
    match User::from_uid(Uid::from_raw(uid)) {
        Ok(Some(user)) => user.name,
        Ok(None)       => format!("uid:{}", uid), // UID válido pero no existe en el sistema
        Err(_)         => format!("uid:{}", uid), // error leyendo /etc/passwd
    }
}
