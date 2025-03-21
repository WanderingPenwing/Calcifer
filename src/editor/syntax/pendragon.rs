use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
	pub fn pendragon() -> Self {
		Syntax {
			language: "Pendragon",
			case_sensitive: true,
			comment: "Nota",
			comment_multiline: ["/*", "*/"],
			keywords: BTreeSet::from([ // rouge
				"Définis", "Modifie", "Tant", 
				"que", "Affiche", "Si", "Demande"
			]),
			types: BTreeSet::from([ //bleu
				"entier", "booleen", "texte"
			]),
			special: BTreeSet::from([ //orange
				"et", "ou", "puis", 
				"plus", "moins", "fois", 
				"divisé", "par", "ouvre", 
				"la", "parenthèse", "ferme", 
				"non", "est", "égal", "supérieur", "inférieur", "à"
			]),
		}
	}
}
