pub enum ScructureSubset {
    COMMERCIAL,
    RESIDENTIAL,
    INDUSTRIAL,
    ARGICULTURAL,
}

pub struct Structure {
    subset: ScructureSubset,
    map: Vec<String>,
}
