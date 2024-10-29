pub fn tri_fusion(v: &mut [u32]) {
    if v.len() <= 1 {
        return;
    } else {
        let m = v.len() / 2;
        tri_fusion(&mut v[0..m]);
        tri_fusion(&mut v[m..]);
        fusionner(v, m);
    }
}

pub fn fusionner(v: &mut [u32], m: usize) {
    // Création d'une copie temporaire pour stocker la fusion
    let mut temp = v.to_vec();
    
    let (mut i, mut j, mut k) = (0, m, 0);
    
    // Fusion des deux moitiés en triant les valeurs
    while i < m && j < v.len() {
        if v[i] < v[j] {
            temp[k] = v[i];
            i += 1;
        } else {
            temp[k] = v[j];
            j += 1;
        }
        k += 1;
    }
    
    // Ajout des éléments restants de la première moitié
    while i < m {
        temp[k] = v[i];
        i += 1;
        k += 1;
    }
    
    // Ajout des éléments restants de la seconde moitié
    while j < v.len() {
        temp[k] = v[j];
        j += 1;
        k += 1;
    }

    // Copier les éléments fusionnés dans `v`;
    v.copy_from_slice(&temp[0..])
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_sort_vec() {
        let mut v = vec![2, 16, 1, 4, 3];
        tri_fusion(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 16]);
    }
}
