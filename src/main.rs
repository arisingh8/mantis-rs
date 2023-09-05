fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mantis_rs::sample::{Squeakr, FileBasedSample};
    use mantis_rs::index::{Index, Mantis};

    type IndexType<'a> = Mantis<'a>;

    use super::*;
    #[test]
    fn test_index() {
        let path = PathBuf::from("/home/ubuntu/neomantis");
        let insert_threads = 2;
        let query_threads = 2;
        let kmer_size = 23;
        let samples = SAMPLES
            .iter()
            .enumerate()
            .map(|(number, &path)| Squeakr::new(format!("{number}"), (path).into()).unwrap());
        let index = IndexType::new(path, insert_threads, query_threads, kmer_size).unwrap();

        for (idx, sample) in samples.enumerate() {
            index
                .insert(sample)
                .expect(&format!("Failed to insert {}", idx));
        }
    }
    const SAMPLES: [&str; 129] = [
        "/mnt/mycephfs/squeakrs/SRR029/SRR029131/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR033/SRR033725/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR033/SRR033726/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR038/SRR038996/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR038/SRR038998/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR038/SRR038999/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR066/SRR066462/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR066/SRR066468/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/000/SRR1012320/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/002/SRR1012322/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/003/SRR1012323/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/005/SRR1012325/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/006/SRR1012326/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/007/SRR1012327/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/009/SRR1012329/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/002/SRR1012332/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/003/SRR1012333/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/005/SRR1012335/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/006/SRR1012336/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/007/SRR1012917/SRR1012917-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/006/SRR1012926/SRR1012926-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/008/SRR1012928/SRR1012928-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/009/SRR1012929/SRR1012929-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/001/SRR1012931/SRR1012931-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/002/SRR1012932/SRR1012932-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/004/SRR1012934/SRR1012934-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/005/SRR1012935/SRR1012935-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/007/SRR1012937/SRR1012937-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/000/SRR1012940/SRR1012940-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/006/SRR1012946/SRR1012946-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/007/SRR1012947/SRR1012947-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR101/004/SRR1013514/SRR1013514-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1032167/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1032179/SRR1032179-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1032180/SRR1032180-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1032181/SRR1032181-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1032182/SRR1032182-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1032184/SRR1032184-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1032187/SRR1032187-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1032188/SRR1032188-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1032189/SRR1032189-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1032190/SRR1032190-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1032887/SRR1032887-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1032888/SRR1032888-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1032892/SRR1032892-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1032911/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1032913/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1032914/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1032917/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1032918/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1032921/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1032926/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1032933/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1032935/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1032948/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1032954/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1032975/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1032977/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1032982/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1032983/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1032995/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1032996/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1033000/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033004/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1033011/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033014/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033016/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033017/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033027/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033035/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1033039/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033047/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1033061/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033074/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033095/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033102/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033106/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033108/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033112/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033118/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1033120/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033122/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033124/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033133/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033142/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033143/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033156/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1033159/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033173/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033175/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033177/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033187/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033194/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033198/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1033200/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1033201/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033202/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033203/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033204/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033205/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033206/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1033211/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033213/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033216/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033217/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033218/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033223/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1033224/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/007/SRR1033227/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033228/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1033229/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1033230/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033232/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1033233/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033235/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/006/SRR1033236/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/008/SRR1033238/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1033239/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1033242/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1033245/k23-min1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/003/SRR1035653/SRR1035653-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/002/SRR1035652/SRR1035652-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1035651/SRR1035651-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1035650/SRR1035650-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/005/SRR1035645/SRR1035645-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/004/SRR1035644/SRR1035644-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/001/SRR1035621/SRR1035621-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/000/SRR1035620/SRR1035620-k23-1.squeakr",
        "/mnt/mycephfs/squeakrs/SRR103/009/SRR1033279/k23-min1.squeakr",
    ];
}