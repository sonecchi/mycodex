# フォークリポジトリ(origin)のmainブランチを本家(upstream)のmainと完全一致させる
git fetch upstream → git switch main → git reset --hard upstream/main でローカルの main は本家と完全一致になる👌
検証: git rev-parse upstream/main と git rev-parse main（さらに origin/main）の SHA が一致すればOK🔍

