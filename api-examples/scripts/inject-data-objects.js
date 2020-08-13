/* global api, hashing, keyring, types, util, window */

const { isJSDocTypeExpression } = require('typescript')

// run this script with:
// yarn script injectDataObjects
//
// or copy and paste the code into the pioneer javascript toolbox at:
// https://testnet.joystream.org/#/js
//
// requires nicaea release+

const script = async ({ api, keyring, types, joy }) => {
  // map must be sorted or we get BadProof error when transaction is submitted and decoded by
  // the node. Make sure they are exported in sorted order
  // As of July-22-2020
  let exported = `
    [["0x024d7e659d98d537e11f584a411a109f823be28c2e33d5fd5bc83705459442d9",{"owner":442,"added_at":{"block":1128903,"time":1591306668000},"type_id":1,"size":219008840,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmc5fAfRUXnHHtXo2dzKG4qMxQU8rz49meRapvmBsGF5cA"}],["0x03d7010faa563d11cad338c43bbac73e3d54369a96385f934a71e454fa307052",{"owner":447,"added_at":{"block":1466693,"time":1593339768000},"type_id":1,"size":288963385,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQDyQFmF3qZ8r1h2Rumd6M1wgykGZTq3h2Zti2UHX91aT"}],["0x05a5d2bc33fcc1d3aff27166a648c86db7e62563bcbe787ec8f9caa58320fc57",{"owner":438,"added_at":{"block":1055902,"time":1590867516000},"type_id":1,"size":427343853,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmT9SqacYty5bdtAtp9DPTiifcX6UDtZbsgZYs29Zt2RGj"}],["0x090721ad1260d7e3a595aa7c006c63142f78d61dc9b98aa02b2f950e368c137e",{"owner":447,"added_at":{"block":1466438,"time":1593338238000},"type_id":1,"size":310674005,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmawgrC6XHUxH461tWjzSRqneZpt9ehP7ApkkxeTKAXFm5"}],["0x09bb6467ed9702401fe14096bfd24f2326b06b97a8147a213bb472560ea7d9fe",{"owner":309,"added_at":{"block":219866,"time":1585804332000},"type_id":1,"size":388639347,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmTbQLFvs6JQpKwHWVV3UdqFxPs8AycXh4pXmAQJfFG5Hv"}],["0x09eeedb5ce5a84a2a03e1e90fcdaad069e2ea4aa48dd2821c31d44b0a13b073e",{"owner":8,"added_at":{"block":835705,"time":1589541222000},"type_id":1,"size":97107,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVjycZ8xJPUtfL46xhgPnZBrX4Zou5Husa99PGH2pxWuG"}],["0x0aed8adce29955d23975b2ab605c1cd78c42ce9aa8e3d5de89b7e4e9f140b092",{"owner":442,"added_at":{"block":1092335,"time":1591086528000},"type_id":1,"size":89227056,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmNqqmoAb8pRCkfsHiEzcxQ7P4DfcbUAgvG6d4Vcn2tANt"}],["0x11766a6bf64bcfb8264dbecb326d75b6f37efdc178037c0dc9e96118088a1e79",{"owner":350,"added_at":{"block":1040723,"time":1590776238006},"type_id":1,"size":108038112,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWVsNMZtR1hEKmk6omV8kDgLMAxYwovd48UePdRFKCkMb"}],["0x11bcfb731670560b4a7f182df74f321c2004724d756a05f57ac3d4968eeea427",{"owner":350,"added_at":{"block":1041073,"time":1590778338000},"type_id":1,"size":76625829,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmdzd86ZgMu7u7A74PXNL3jAsj4LydLEkejckSHDxnGDwn"}],["0x1569df4aa068b69fe2b88fcd04a9a1939eaec11a45e15068f2d357d29124735b",{"owner":350,"added_at":{"block":952596,"time":1590245130000},"type_id":1,"size":83605504,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWMQgA13brwFkWojPAQY8Jt48fxp8GpuDsAUfMGh2RuCE"}],["0x1630bbc4eb9cc803dfb7e6f445baff6938cf3e86cbcce6a4308375bd9bcb17b8",{"owner":442,"added_at":{"block":1129261,"time":1591308834000},"type_id":1,"size":290289463,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmTcMFK8PEdK9ayv825GT8AP6RfKQq4eYSnnDqY61XWGGi"}],["0x19de0fdc6ef77c6afb52ee1e1f6f5a70557442725852385d34a1537857d78793",{"owner":350,"added_at":{"block":1040903,"time":1590777318000},"type_id":1,"size":8388608,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZofxmdtoUYiDRM2LopoBgoaf8QuSdRWDS4mxMSkgw6Qb"}],["0x230546c3c70f0e892adf19be590e8aace2a091f6a6bf6e9e4e84ceccf8f84c57",{"owner":336,"added_at":{"block":1135451,"time":1591346100000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0x24949a52872f06ef61ff3a453e14ffdd358518a434e20f46819620c7d8fd788c",{"owner":6,"added_at":{"block":30261,"time":1584641082000},"type_id":1,"size":11703644,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbbK8TSs4YKo25uaTQDN16GWcUpGkTay6hF2jL4aLyy4z"}],["0x28deb34402e47352adea11da480711b04190e3e2d4cd6fe6cbc2fc1823d4076a",{"owner":432,"added_at":{"block":1213031,"time":1591813500000},"type_id":1,"size":22071878,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZcrf8VHgg78NwG7zgwYgBB4kHo5DM3GmPNdjDBe971rk"}],["0x2a191bdf05d03a11ba557241dd0d94e820144c1da7aa37bf4c75cc7207af4c88",{"owner":442,"added_at":{"block":1124157,"time":1591278024000},"type_id":1,"size":239017927,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVpASwjCBiMQm2uHoamxA51pqBuFDzXmV5oKAsWN7xzqV"}],["0x2b88e65e69fa58f983e5db6ce282783d320bda37faf730db959f841bf1777834",{"owner":447,"added_at":{"block":1466836,"time":1593340626000},"type_id":1,"size":67518865,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXsFsAaoSxzEtBtzRZ8f59NicapmV98PEReMVGtycumaV"}],["0x309c38c4216d23dc59d8f2747d6c7a3fca4f31e68f6cbe5d22afb704f22edecf",{"owner":447,"added_at":{"block":1367174,"time":1592740608000},"type_id":1,"size":355856562,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmeSnJjdUocf4pSifABZByHvNUbDjRCfS45bYHmQFq9GRx"}],["0x30e517661b82e1c774a659cd2d7ed5c33be6ac0eba95a74596c05ff912829f4c",{"owner":442,"added_at":{"block":1128414,"time":1591303674000},"type_id":1,"size":219008840,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmc5fAfRUXnHHtXo2dzKG4qMxQU8rz49meRapvmBsGF5cA"}],["0x31a7436a29d8daa4fa7a20a038b26614964584846ea92f3e5f5af543be59a900",{"owner":442,"added_at":{"block":1121633,"time":1591262862000},"type_id":1,"size":171629454,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP5U82G3zBS7Z2nY3M877knEHjMBqZw5v6Y2z6vpihFaG"}],["0x32c8a93d73e14981ad910e6dfb19c83212fbeccbbc773cf3b35e33ac29ee6dbe",{"owner":442,"added_at":{"block":1129511,"time":1591310376000},"type_id":1,"size":381032453,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXv5w8M34dLSN31KoEsbsVHtqpi7yYgxxEdSzvS5ArN5T"}],["0x339d6574987cd3802c89efce1d93a606367ac5154d4e91c749ede761a918e4c5",{"owner":442,"added_at":{"block":1098823,"time":1591125474000},"type_id":1,"size":61109974,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPXxuMcEvZstbaHEy8CqCramJ8ZJRq2faNut8igmvRaDA"}],["0x36ac61276f1ffb56f65f54d47595457b669f6afec86b800dd7fbf76423c8df9a",{"owner":335,"added_at":{"block":281214,"time":1586187438000},"type_id":1,"size":486671229,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQUPSGGt9zQEUHHgvLsUrKWBGRbJZwLDgN9Ds1zGgHa3Q"}],["0x3b3318f4949850d00e0bb53abc60b17bba6918b56f1848f3c04bc8e32ec62a01",{"owner":442,"added_at":{"block":1128625,"time":1591304952000},"type_id":1,"size":476300317,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmY3BVExv59DY24A6kLvmayBNgGKudhdaKcCbTvLj8BXyh"}],["0x3d9d6721a05e68875a598c3b754e28ab466fad2e5d2bec5190019f8eb86a9fbd",{"owner":447,"added_at":{"block":1367042,"time":1592739816000},"type_id":1,"size":464135764,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmTPjGsmJMq66rHtQMtUu1UiEnTRght9NW7PuuvPGnNPrV"}],["0x4202b8da5f8b18d2ebe74eeb333f00ed22326cea14fad0c5db14521bb90cef4b",{"owner":442,"added_at":{"block":1129332,"time":1591309266000},"type_id":1,"size":296289584,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVP9m885U84WjFqTH5Z7r17hrtwK1VJ9fbfriMw11Zmhi"}],["0x42e4b57bb3bc6c7c72f7e1e1c3dba164f301799ebc62f52d94ff110afbc58daf",{"owner":442,"added_at":{"block":1129185,"time":1591308366000},"type_id":1,"size":228968517,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmW1ogMZu3Pp6W9empkK3SxceqcoKHas7rg3iybYoUA1mc"}],["0x44792a5aa5b3753afa1158c47f9975b3e7abbbbb213d1e91f024c3e1f688c68a",{"owner":9,"added_at":{"block":1137835,"time":1591360464000},"type_id":1,"size":183564493,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZExcmZdfyLh1en5ozAejPaN76QTPAH61HCqRjgnzFq65"}],["0x4800d425fa2591edb2b936b4f3078570ee3740c2434a99a3f96e85afd37bcaa5",{"owner":442,"added_at":{"block":1129061,"time":1591307622000},"type_id":1,"size":323491639,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmf86Wu4uHrnNUAFeM8CSyUprCfrwrKE1Ejg8AJGqe5GHv"}],["0x48495b3033d497e76298e9301db064d1d36d1b845154e0d4baa06c150a9bbe4d",{"owner":442,"added_at":{"block":1129821,"time":1591312236000},"type_id":1,"size":16788182,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmSmEiRfh2x2CnksELXXfd5pNfFHh7uzAvx57dCu42JZYB"}],["0x4964c7e73f4129f5f541b4ac2713b8d10922cd2249fa2577978a775389455f6c",{"owner":442,"added_at":{"block":1130032,"time":1591313502000},"type_id":1,"size":41396854,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmNsERXvLpqKFWzc18zBVwj2XzQtPq3uGjRhdJuEjdCpBa"}],["0x4d37449528ca4e54b3a56d51ee0bd3dae289dfda0d683c8d52bf60f804b24474",{"owner":350,"added_at":{"block":1040719,"time":1590776214000},"type_id":1,"size":116963310,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmevuPnBLNzbcqCnAjEf38GCPyFFWDWAjVEXpiVxMFggcM"}],["0x4e5b77b2b26542c352bc9d67f2826739f7947a8dfe8780c0da7ef5a578325c16",{"owner":442,"added_at":{"block":1126921,"time":1591294632000},"type_id":1,"size":454042427,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmTrvQUtZgjjzQUivzrBYPZH4ZmUfSGDMy7kjgBLvJu8cL"}],["0x4f88cb65b21c9960bfbb29941258643a41d279f85e7e9d3e361dfb007d2082ad",{"owner":448,"added_at":{"block":1426221,"time":1593096558000},"type_id":1,"size":512691061,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmTBtR7zRCu9QVnKR16Rh68KPgENWsm1miLe8rWxx3s3Rd"}],["0x5339a2ad58454d0f2f56fe19a402555197cf0cef6090d38be5991cfd7560a29f",{"owner":448,"added_at":{"block":1426315,"time":1593097122000},"type_id":1,"size":48401831,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWZKK2tduQHWh9WdekCzezogikZ2N17Vkjx59EqKKLMhj"}],["0x5487981d021b38d574aa1585b80dbca4e0a06356f654a9fb388813dc73a45afc",{"owner":442,"added_at":{"block":1126059,"time":1591289454000},"type_id":1,"size":228190053,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmaV45bM5unHC2eavW2wrXcPkk1eteXFM5ohKdxact37yi"}],["0x555112d6f5c0668b2b6a6e1fc4140dd9678cf3ede7d8167ac3281d8707fa510a",{"owner":350,"added_at":{"block":1040822,"time":1590776832000},"type_id":1,"size":115602785,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmeXbHshnx8MrAMSn8BwPv3VgW9HhyneuyjXVhJQp7AEzr"}],["0x558d4f1851d7a031aa77e02a47edcbf3769fbfb15d91356359c7f0920d362204",{"owner":438,"added_at":{"block":1067485,"time":1590937188000},"type_id":1,"size":466914342,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQPFijUtKGY2h7TLMoKS43kCTvbNtBJt5zhkWB2eXNcFv"}],["0x567126390559878911908d9014d16970ca3a8fbfafb73ca58ea2266e4df92cda",{"owner":447,"added_at":{"block":1466727,"time":1593339972000},"type_id":1,"size":175136347,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmcsnrCbWBdw1NbqgHb2NpEKpGg1jmZQScAiYkocxVxJKT"}],["0x57fbba6e14080b7603a37d2f72661a0e50e8bfc75d2016751a0fac2e0e0c5ce8",{"owner":442,"added_at":{"block":1129257,"time":1591308810000},"type_id":1,"size":377630545,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmdLgMHn8uKS3bZM7VzV7BKxGK9RB5mpwfZpkUM9WNDkGS"}],["0x5c0d046280e9656772ec5e716c2849c6ac97a27d7e7f5a81f536614d6f26f401",{"owner":442,"added_at":{"block":1092298,"time":1591086306000},"type_id":1,"size":103571823,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbPwLcE31iuUeFaiPQGnrTPWEQpjRFSTrxf5GErNGVrLR"}],["0x627e905c3f1cd963bd6c00d61a742b88ce753139c7a245ed7c5afcbacd0fa79e",{"owner":350,"added_at":{"block":978132,"time":1590398916000},"type_id":1,"size":14055552,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXCoAJrffVv4Wo9cpwYmRD1Wan1p1Ae8q9ayFgwNBXZn3"}],["0x6860c305bdb1b50bb16cac8722a661d22180f9a7aabf26c65e07c1c7391c0a9d",{"owner":442,"added_at":{"block":1111030,"time":1591198980000},"type_id":1,"size":171629454,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP5U82G3zBS7Z2nY3M877knEHjMBqZw5v6Y2z6vpihFaG"}],["0x690f17fcf72ef1411d446671abe79a54cd636f5fd861ec7edec1734c76cd76cb",{"owner":442,"added_at":{"block":1081934,"time":1591024050000},"type_id":1,"size":77047118,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQ8CMEaCgLtGTW9mfHDtScJev4EpX8fsaSjDohKvvHHZr"}],["0x6b37a30969e1179425bf6aa8981beb7f8eeb9db38da5e939749fc50032908cac",{"owner":442,"added_at":{"block":1110501,"time":1591195764000},"type_id":1,"size":143177220,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmcnCxnLXZVrkGGKMRt5dQqbp3nsbCWdMgCw8oTWhJJnqV"}],["0x6b581bb1fa3f25bf0fe52d08cc2cfce9d84371154556b9ebe9ad4778e42b24f0",{"owner":309,"added_at":{"block":10882,"time":1584522906000},"type_id":1,"size":357059329,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP4Q9bD1iJomLk5cmWseATeYiQpTf2mTajvrsH5wCAnGv"}],["0x6bd69751fecccf320548f4b577b13c0e23a59f826f8a408c5a7ec11f03328ce2",{"owner":442,"added_at":{"block":1082012,"time":1591024518000},"type_id":1,"size":42754567,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmagdA7Zf9Hprj2ZrYEBM2WU1WtZhjh5TMtDgCKV2mov3e"}],["0x6c16eda9fd31abbd626c155fa85990f5061e87d4b024599d9446e0a2363f6549",{"owner":336,"added_at":{"block":819861,"time":1589445528000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0x6cb1958d0fe6cc59eec2722574175b0dabe7d431795b7bc5e80290fcfedf25e5",{"owner":442,"added_at":{"block":1121846,"time":1591264140000},"type_id":1,"size":214382445,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbgqP1W2wmnf7qWmdoo5FKxyEbygoudcSekYmJorUyhdL"}],["0x6db58a7fe253876c2aea44a3093c6db043b34cc1a16f0ab396b039c410042929",{"owner":442,"added_at":{"block":1110793,"time":1591197534000},"type_id":1,"size":171629454,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP5U82G3zBS7Z2nY3M877knEHjMBqZw5v6Y2z6vpihFaG"}],["0x6f8767696e9e07a4691126eeb5cf790950ff82e32daff1d8be13d9aa5c5cd8d5",{"owner":336,"added_at":{"block":821397,"time":1589454804000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0x71e3020c5f4181dfb646bf2a1a1da38a536fc2cafcfaffce989093d247ef6515",{"owner":309,"added_at":{"block":4059,"time":1584481830000},"type_id":1,"size":297638125,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXTJ8JS2L4DLmYSZcjLR9KZtqCKTwTmTT4JcenPwHYzsC"}],["0x73b14241a22c374a2b27161a2e259705459ce1cbee979b4f27a910c6befa5873",{"owner":442,"added_at":{"block":1126289,"time":1591290834000},"type_id":1,"size":303724484,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmUkjHiHdSG5kARgZQrCWZeyMVKBfxpT4m1DPEk3LMsRSV"}],["0x76318fc316171d4197e71a06e18df41f5ebc7a1b4b98be15a1f6d4721eca67b6",{"owner":9,"added_at":{"block":10800,"time":1584522414000},"type_id":1,"size":102256050,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmW8j5XdoKewAzJJvq72iRhXK4nKfRzB5eQXGFjnJTGgqS"}],["0x774d61799ad74137d18b0eab532bcdea6b82c8bd1b6a646d861b95d7b8c6ddbe",{"owner":448,"added_at":{"block":1425677,"time":1593093288000},"type_id":1,"size":383882912,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVDxMygHdBYdaYtf3g7ZigyCvxusqEwQdcfNbDxwMCndR"}],["0x7a3859e8742d9bf85b5fc0bcbf5fd86aaecf58119f3a33dd1622d43ab8182510",{"owner":350,"added_at":{"block":952082,"time":1590242046000},"type_id":1,"size":85057536,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQ9agG1gMb7U6w4amyENMTHru34AwZhSgPZCzTAbWJ2Mb"}],["0x7c0a51e41fccf3adca2ca7fdb4492ddd80a3c8716ac04ae408a0a839778405ec",{"owner":336,"added_at":{"block":938132,"time":1590158118000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0x7da45cb1e508b802d7eed7f2f07fc8ebea803266027a204ddc219626acbe005f",{"owner":2,"added_at":{"block":1812377,"time":1595417754000},"type_id":1,"size":125787601,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmUmeRNYYBWQtju26msXXsxfTytBbKmwTpPdR4z5Vk8L3U"}],["0x7e496ba7bbb89262263a2e9978bff5f151c1045222245c2434b9c3411c68373f",{"owner":309,"added_at":{"block":250877,"time":1585994772000},"type_id":1,"size":415862058,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXMjGjMaUzhADTUkCwMx9zcwCbAHUqhg2fnkRapCfLgbJ"}],["0x7e8798b65aeb97d80f15614f4fbfd1ccc6ea455e450381c8effe67f5af473cd6",{"owner":335,"added_at":{"block":252882,"time":1586007636007},"type_id":1,"size":278383205,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmRtgQTyduMCqGTnRUy8b52h53CATVMMfjqxjFC66Td3HS"}],["0x85ab1f59f7268995b96cb5eda25f69aea13ce8c7adb18fc2150cac1accb34e10",{"owner":442,"added_at":{"block":1123966,"time":1591276878000},"type_id":1,"size":185849059,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVP24JciSRNNd3wUELKeWJR1q3dJK2thbKu6Pky2Y4Zf3"}],["0x89a0a48541889c2cb98592e715225facaced548ea7523e089a593409904a66e7",{"owner":447,"added_at":{"block":1466899,"time":1593341004000},"type_id":1,"size":90491542,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmR2WuxrferxegFHCyMoJiByAwzW5n4bt7exwaaMB766qa"}],["0x89c316cf198daf7df103a82bb5ee86116acfb8a6ca84de01a53449f8c5fed3c7",{"owner":442,"added_at":{"block":1110622,"time":1591196496000},"type_id":1,"size":247006015,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVDt1MFxNS8r416RkeEadadzhfFomPby18wcgbCQ9RBxL"}],["0x8ca510b0578ab7e688db30816f783ad78bfe4323cfe045beff1d11244173f547",{"owner":442,"added_at":{"block":1098712,"time":1591124808000},"type_id":1,"size":62038020,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZ63mpsT6tFbr2fx7BxcQRM9p7VDHBVLCtCZQ4Cw68kef"}],["0x8ee1dbbfa90034a258ee8296d794baaf7ffeabf1fa43ed075ddd56ae84a7769d",{"owner":309,"added_at":{"block":295617,"time":1586276886000},"type_id":1,"size":305573424,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZGpGTC8yoUvR7WPmNB5FC8ZJCikrroG6LJU9qtZjzFcr"}],["0x8eebd480e55d8971744d8d35839f5092c2dc5a57d8b96e8c9e1641ed5446df53",{"owner":442,"added_at":{"block":1127271,"time":1591296738000},"type_id":1,"size":296167714,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmSdGGSoo7htcyRA93VmGWWv6PezcQFTQ7hzoG7WTz9nat"}],["0x8f32787bcb8ad38be82e9595a44b8adcd5e5271f688dcc61d29f796db5cad19e",{"owner":309,"added_at":{"block":80121,"time":1584946890000},"type_id":1,"size":358383122,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmZszxaTJuyLE7bsWimKmA6EnWq7CmCDUunQHPyZ6kyJYM"}],["0x8f7948e5e046024dbec83d4fac4188348e32f3ce3ef8b26561326aad4a1150a5",{"owner":306,"added_at":{"block":399005,"time":1586904192000},"type_id":1,"size":59359868,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPvgMP99gof4RYmcSMEciN3foGmWnMzefUpZAGCCJ8GX8"}],["0x916b6fc96f3d00dd90fccc4618c67178cda61885330c78019fee3d34ffd3e148",{"owner":309,"added_at":{"block":122598,"time":1585207182000},"type_id":1,"size":293299508,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmT21sEqJxDK2LYfZckjyaXWorkNYjuSLKyMrQVt5nnEXX"}],["0x91883d2a207ee1f772f3ec7f48864fd3931500f37bfe8bf0664e3829560a5d5e",{"owner":350,"added_at":{"block":452900,"time":1587232986000},"type_id":1,"size":8808959,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmSiDroNEFVwjmnvqguq1Ke6EF77JixsUg1negSWsZtfrt"}],["0x91ae12a19f8ba2461e53d8c0de653c64330996c3e2b93234d8f5db6caae5ba10",{"owner":442,"added_at":{"block":1128145,"time":1591301982000},"type_id":1,"size":296167714,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmSdGGSoo7htcyRA93VmGWWv6PezcQFTQ7hzoG7WTz9nat"}],["0x98a2cae0e026c32ef02ac9513d7160a78dc9b3e587b6d55fada211ce811611df",{"owner":447,"added_at":{"block":1466522,"time":1593338742000},"type_id":1,"size":406931374,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmdUmdJLFH3SxSUaW46VuN47wo888hFSGt9RP7dBZKHJFe"}],["0x990889a2b149410e4d3576ec5af732843cd38552238d021c614a811e32ac997e",{"owner":442,"added_at":{"block":1128091,"time":1591301658000},"type_id":1,"size":466729213,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmfLASmYcoZwKzqemWDH55XLMvSqobb2prZA6xmq6T23Cc"}],["0x9d75a21ff370cb8b1e74e88d58b5e11a928ac1c48f6b7098e1d25e6df4ca2eee",{"owner":350,"added_at":{"block":1040980,"time":1590777780000},"type_id":1,"size":104637904,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmNjmKwksF2WabmA4NyLiyk7h5LZpiRqryQV87JzL4M26x"}],["0x9ed8ded941839a77e2ac1ba931d5a3f22bf142794c3b4297d1d65bac92f9d739",{"owner":442,"added_at":{"block":1125491,"time":1591286034000},"type_id":1,"size":377595483,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmcrf9hMvWqzoAEWH2fnAr1k21jmd1hjvKiTp7BNCJDY73"}],["0x9edacefe7b92331829e93653e5e6c470a73dd6d5eb9e8bb3e6504ad97c6b19b2",{"owner":442,"added_at":{"block":1110425,"time":1591195296000},"type_id":1,"size":211637438,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmRmJPWavbxRboXoUvVSkgcL8vCQdxpdtifdyLuuzqSNDA"}],["0xa25d85c5e547dbfbe85edf476b7a3fcecaff3e7a1a156a0ff1c9ace0089bca43",{"owner":442,"added_at":{"block":1128755,"time":1591305780000},"type_id":1,"size":487027735,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmX1uCVbqhjxrVa6ZNp2q9XCaMrTTBEZVUjmq4xdPRjGQG"}],["0xa45878ab69b49ed57ffefaa524548c5c69d05661202f01073c94ea14185f7f5c",{"owner":9,"added_at":{"block":53411,"time":1584783462000},"type_id":1,"size":211387424,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmdpygXdKmjgCQNB7wGN7wzhGHD8gnFAsuiU77TTFCbpFy"}],["0xa575d1ee097acdc99989b4d8c0f4ee56bf472e793f5c36ab552ce18ca68109fc",{"owner":442,"added_at":{"block":1127098,"time":1591295694000},"type_id":1,"size":478905799,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXEbCMrBBr56o3Hyu1PUf35kW6UWmN2jaHFJAZMJKTew2"}],["0xab2a6a80321c2b4409c2804af9ac1fe1a8434419d6be1266e3df1207e2470ec4",{"owner":442,"added_at":{"block":1092258,"time":1591086066000},"type_id":1,"size":96442249,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP7fZ4ERh7JzHqVxveE2ZPv7mX46Lh8iAQk7KiMtVEo2g"}],["0xab4109dcfac767e9b557cef22ce726cc2a9af0fd58749e7dff7b9a431e130561",{"owner":448,"added_at":{"block":1426376,"time":1593097488000},"type_id":1,"size":48401831,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWZKK2tduQHWh9WdekCzezogikZ2N17Vkjx59EqKKLMhj"}],["0xac3480f720c1c24c7aafcb13c707be44badcf646690ff7bdbede011b28636d7b",{"owner":430,"added_at":{"block":962539,"time":1590304926000},"type_id":1,"size":145386199,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmeg2CZPpFhWwU2f2dcC8RQsmS9xuiUFbox6twFFvY2b16"}],["0xac9b568a6b7fb3403ba12eb15326740ccbfc1487766e62a1df239867a779f755",{"owner":309,"added_at":{"block":385966,"time":1586825580000},"type_id":1,"size":355621626,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPtSKii2xQ57DUNeHCxrGnuGtrSFHGBZzcvJauyb62Zo3"}],["0xadf8a4933f69fe51a9b16cd4310389ba8efadc8bed078c30f61752f5ba4ca625",{"owner":442,"added_at":{"block":1129663,"time":1591311288000},"type_id":1,"size":285552738,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPu36d9TJorVTLog9MQcXFsuhLYwjvy4o4n45VRj3NZeg"}],["0xb52a6681e9bc13208bc4a9c1238b142b83eb793c8e63aa95628dc31e5d134c50",{"owner":309,"added_at":{"block":178824,"time":1585551600000},"type_id":1,"size":415473786,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbruxNcR2AnCaW6UQU5gGXUZH78kePhcw8d8PTz56rVBb"}],["0xb5e6aa0602d416581017f69bd1f4f4a6d81122ac6d1d6a1e1572b5ac6ecda230",{"owner":438,"added_at":{"block":1066911,"time":1590933744000},"type_id":1,"size":466914342,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQPFijUtKGY2h7TLMoKS43kCTvbNtBJt5zhkWB2eXNcFv"}],["0xb73cfa311d157640033d60611d87ffd9232a0e6ef0b80e0302ac90879b2619c0",{"owner":442,"added_at":{"block":1129388,"time":1591309626000},"type_id":1,"size":178952334,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmUbULJeFkgUTBAzChSPigpN6D9VeikkHLvijtrE1AskBu"}],["0xb75caf11de2bef62de71b1a13be82194451fcab7109dba53c0f7a4adf084cd75",{"owner":140,"added_at":{"block":308380,"time":1586355066000},"type_id":1,"size":30770348,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVuLj5TrBBwuVTNoW5eRXrf7xsmsCXrKR469hfv81kihR"}],["0xb7c6803a781aaf175846cf4a3d9bf61303be3cb86e2e4f357ca6d08dacbc55ec",{"owner":336,"added_at":{"block":820868,"time":1589451630000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0xb964415073fa910b2e9ade2edb161ba32183502986543a1020c19c03af4126b7",{"owner":442,"added_at":{"block":1129456,"time":1591310046000},"type_id":1,"size":274204288,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPVfNCnVrbCxGxhR1BZrrjcv9WH8zwEKXXznGSDZT8XfZ"}],["0xbb904988c4888a74248f2d18d664d95e9bd9b6392c8a709064c8d54a0cf2b1ef",{"owner":309,"added_at":{"block":23619,"time":1584600726000},"type_id":1,"size":317341899,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmUtsmwDF8HrrNXKKTfWYzXFznt5rCEbUe8Q2gGJ6ZTiab"}],["0xbc10f23b830d39018dc8f60f2434d7db3d14262364b3d4e8617a9e8b9ada50a7",{"owner":442,"added_at":{"block":1126767,"time":1591293708000},"type_id":1,"size":256241998,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmYPfooy3YLKxsLfScbmZWNJy4VyAcK5WkfqwjdsWzL7GG"}],["0xbf6c9db08a2f0dd7b5b3966a8f8d8ab312c2965d25867a1ab7463ffc6fece928",{"owner":441,"added_at":{"block":1114794,"time":1591221714000},"type_id":1,"size":55708471,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbfXW6JafVg9dUesazR2wkJq59cCw6eSo8TLUY5Q1rYzQ"}],["0xc17cc1f1a2723bfdc468d624b4b223c0fd31376877e88b7515ec35dec29e7ae2",{"owner":7,"added_at":{"block":1594924,"time":1594110738000},"type_id":1,"size":513795950,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmSbdvdc46qE4k8HftqoDW942Ay777RSfhQ6PcGfbkyA5f"}],["0xc2609ab0d956d2daea48f45c678c742b7bdc35d7b03abae4b3c5895a9e1523f8",{"owner":350,"added_at":{"block":1040904,"time":1590777324000},"type_id":1,"size":109999676,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qma2HgfkVZNXiNQwUNZSCr6k4Vvnebc8XVJyw3tEF8EMNi"}],["0xc2924d93aa476e303ecb8c76cc9473a608cdbfb1d40722d8557d0a37764345a9",{"owner":442,"added_at":{"block":1099325,"time":1591128486000},"type_id":1,"size":214961405,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmT138QMCCzJxMhYnFZcvLf8Cs1Joptoy9wtoUqMFWsZQr"}],["0xc4a1e8f7c0d7ec4f0616083c5bcf1dbf1277560910dfeb5417dea3764a64390f",{"owner":336,"added_at":{"block":1136287,"time":1591351116000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0xc6649ad85ef2722b6088f37b83c79e0181f372d27a80169e2d70e9e382f1c110",{"owner":442,"added_at":{"block":1126872,"time":1591294338000},"type_id":1,"size":323293926,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmdB7mitLeNQ6WZkwBQXVZtd7jDNU713XAfsgDmijt49A3"}],["0xc87a2a87b0cd125a2fbc222f3855caaa1fdbe2149f55c8670b3ddaeb873f309b",{"owner":311,"added_at":{"block":40871,"time":1584705954001},"type_id":1,"size":23677745,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVfnwL5bbv7veTYwS13zUny7Qt22cGei7HCBiunmM64xm"}],["0xc9b3f0c761e8c26a9e75f70628fcebf838fb85771c07874abef3ac83852deaa8",{"owner":309,"added_at":{"block":10038,"time":1584517812000},"type_id":1,"size":53550842,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmcZmcm6xQY3wmiB8mf4aLKe2XzqeEedvWZoszBEKGpfpG"}],["0xc9dd0359955ae104e36a2267a3e0f42fc95d408658f05b78671e43c1133bd549",{"owner":309,"added_at":{"block":40542,"time":1584703902000},"type_id":1,"size":101120291,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP5Xg7JLyt6aKZy9AEriP9S6N2D4BL3kA3viWu3M39U79"}],["0xca99d0df631f2f57ceaf7d96954086fc93182fde0c98bef764def13377fba8dd",{"owner":442,"added_at":{"block":1121678,"time":1591263132000},"type_id":1,"size":320771545,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmUgaKnr5H5KFvUeddmcQb37pBH5y7fF3ALavTFh5UaGmK"}],["0xcbb5df475a3bacac3435067d597fa27c1eb723bfdd713a94cdb3b8a48b9278f9",{"owner":442,"added_at":{"block":1129080,"time":1591307736000},"type_id":1,"size":402651528,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmbMePgwenk4swC5qYBUCVDa116CavRJFg3XZkbr2WDttx"}],["0xcc2cc5ff13676cf96bd31b2e40dc7fc06b1be2072646ac7e9f30147ae3606c1f",{"owner":442,"added_at":{"block":1127163,"time":1591296084000},"type_id":1,"size":277538135,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmR8GkpSW6BCw3idWrQdSeitcdQSt55XaTja2TzLeBezwp"}],["0xccd2906754b6e3dd4ead4987488aaf9d5f869ede35005176e88c28109064953a",{"owner":442,"added_at":{"block":1129364,"time":1591309464000},"type_id":1,"size":351058986,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQEhoPJ1RR2opA4h99FhMJWEvNVwn4aijZpBzyk3iTtLC"}],["0xcd48e94fe4caa413352e548452a921abf2bb8b72a8bf1e966bf16bcae2b3c6ee",{"owner":442,"added_at":{"block":1098883,"time":1591125834043},"type_id":1,"size":61507811,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmV4zNzSWQVJWjDriAFLeevwHQvyTpw1HGcqPbg4DNheBz"}],["0xcde4eefdbb74b50ec27cc0737a3ff40e5199eb3ce4a3df0250af6fc2b72a0fa4",{"owner":8,"added_at":{"block":940641,"time":1590173256000},"type_id":1,"size":80328,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmY8bC5GWvkhS7W5den8p2Lh2WpvVV4De7KzikYgHTK6nT"}],["0xd080a437811cb52706e66961d50552f930e4874579bf7d57967a37bbeba5db19",{"owner":336,"added_at":{"block":1135332,"time":1591345386000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0xd1652d0a713a94886774f4ab17544806d2a4e6a5cdb0fc3ec15b2560f28af032",{"owner":442,"added_at":{"block":1129765,"time":1591311900000},"type_id":1,"size":308527570,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmRsKoz7nNuoxMpHR5NqoTgzMyxgsnkiSsbuXcL7LdHV2h"}],["0xd3087b8fd18d5a4ec73ce24149216a5d1658e82f1ddeadd6b9525675bb34e340",{"owner":441,"added_at":{"block":1080090,"time":1591012986000},"type_id":1,"size":19727184,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVh9K2yDuvvgbC1g9Rh5pP4jJZTRWXjkaFw1vdPvLrdut"}],["0xd7ec06e442bba27dcfa2c834964f37126d470a3dd1b0374567a18af239474e50",{"owner":442,"added_at":{"block":1127347,"time":1591297194000},"type_id":1,"size":457868208,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVu6FrVaP8jLZiCG6VecKaUNuCB24FHdEj7MjSJABvey9"}],["0xd97b44423c45fe5396963b57ed44f6a91a839160bad3e554bf08ef19fb583127",{"owner":447,"added_at":{"block":1466620,"time":1593339330000},"type_id":1,"size":200420923,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmYS4JAwG45SvDEC5AM9Hzv1dfzPoXoBq1N9aTbT3bEs86"}],["0xdb8f0622b7fc638933b13a15260a09501d01ffc6a23624bd36317b0805d963b7",{"owner":350,"added_at":{"block":1041080,"time":1590778380000},"type_id":1,"size":99136786,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWLgJN8o8YK361jTxCCf96mbubgMCmMfNx5nEYDcF5hs3"}],["0xdffda551959482486ce30e7739e2eb3b8006b27faba8ed656fc643b2c5f26851",{"owner":449,"added_at":{"block":1683972,"time":1594646064000},"type_id":1,"size":1675105,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmRwHXmuqS2rdfqnZdcUGbB3bM5UM6jkvdghgSr9coJ2A4"}],["0xe0be443d0d8967f35d02bb9dd7a4d01a3d31b57115ca31ba5174647a58e62d00",{"owner":447,"added_at":{"block":1466594,"time":1593339174000},"type_id":1,"size":125974946,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qma2ZYN1UkF9TRaJxqGuN5Ksg1Mg5rxbQAmQ46PDSipd3M"}],["0xe24cda3dc434513e9cad5888d8466f0ecad2924db10c9788fb56cbc811cb2ec5",{"owner":442,"added_at":{"block":1125805,"time":1591287930000},"type_id":1,"size":233980798,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPcFojVSw9E1UTwLCk53sG9cyM8wo2ZAHTt9bV3o5PcnT"}],["0xe44c4c26e4804a54d3fc3766ae6da16fe3f25ccaaa9a95d3c122fbc0af74dc2a",{"owner":447,"added_at":{"block":1466769,"time":1593340224000},"type_id":1,"size":59703371,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmNw2pRA32PGTxVeq7ktaM9ttUHkj5P3is3QLqamLCJ7Bk"}],["0xe6b072d81f0ca33716ddf332e9460ba177650a25dff7e3126e23a7278cffd335",{"owner":350,"added_at":{"block":1040949,"time":1590777594000},"type_id":1,"size":134414330,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmajAfBPnymCpDKPhywX7abeT9kMfFiYsDX9kAvJ3FRcpv"}],["0xe7547e765c44a27cf3ff6e367f5ae3db931927adea06736e95d33bb93fc19ec8",{"owner":442,"added_at":{"block":1128936,"time":1591306872000},"type_id":1,"size":483398954,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWAnhLsrVUAExgNDoNCREuXFUfnn9tJ4tqVm3cyKSuJjC"}],["0xe7ffa7b44f4641b0cc9ed9c0d6d9d42972d745df451bb635dd25d46ff2844172",{"owner":336,"added_at":{"block":421500,"time":1587043908000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0xe8208ac9f0280e6db102b0d9f1e2d82fecbfe0ab99dcda32bf465cdd04ead0a4",{"owner":442,"added_at":{"block":1129630,"time":1591311090000},"type_id":1,"size":491998570,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmWPaYEBopBh2i4RYumohbTXdYYYMMtNTNPAdEhzejU5Ad"}],["0xe84290c050e44d103806bcfb37441adb7808c7044d80b01dd1c3402be643513d",{"owner":309,"added_at":{"block":9919,"time":1584517098000},"type_id":1,"size":337713518,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmPdYHJGDZ5ucEzC8hmmmjUvtzEdKPEnUe4biwpfjahMmJ"}],["0xecc2c164f4d78b61cda6c9f7e414aa2fb4a102356f0e46d14f4a625c5e7afdd5",{"owner":309,"added_at":{"block":41276,"time":1584708432000},"type_id":1,"size":428470328,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmXcaiCAejKga3CYoKqLPKaoxFYemdwCjme2n3PLUwZLvz"}],["0xef41f0a04d3a2d1f29e741fd40abba35ce6d095da54202e49cb77be0404d2bc1",{"owner":447,"added_at":{"block":1466867,"time":1593340812000},"type_id":1,"size":60474460,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmRdHAkcv1ijiooTEBfzstosGxpKNJP55Hrkj5ooBaLCy6"}],["0xf0c3624748e1b33f55c2b2c65055af4ecb6458a072690bd3fedf0fc3c9dda1fb",{"owner":442,"added_at":{"block":1129493,"time":1591310268000},"type_id":1,"size":466951798,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"Qmc7wwKSPBB73MPPEPDnkvphp2uywBAPEoDTjSZN6hZWgP"}],["0xf2af183bfdd67f74e7c190fe330b436d45dc6f1f3af540ca3f5f008c2f9ba027",{"owner":442,"added_at":{"block":1081980,"time":1591024326000},"type_id":1,"size":77047118,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmQ8CMEaCgLtGTW9mfHDtScJev4EpX8fsaSjDohKvvHHZr"}],["0xf3f0a37a878828f8461d269fcf188a47ae97378d58822a1e249ee105c304deab",{"owner":442,"added_at":{"block":1127682,"time":1591299204001},"type_id":1,"size":457868208,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVu6FrVaP8jLZiCG6VecKaUNuCB24FHdEj7MjSJABvey9"}],["0xf9a250565c9d46659ea8be801f324dbeae0584fe80088a61fb2a1fdfb5e25586",{"owner":442,"added_at":{"block":1110733,"time":1591197168000},"type_id":1,"size":247006015,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVDt1MFxNS8r416RkeEadadzhfFomPby18wcgbCQ9RBxL"}],["0xfbe7ea222bdacdeb93082bfec4044f02debe9131488aca1440318b7d1f158b9c",{"owner":336,"added_at":{"block":1135064,"time":1591343772000},"type_id":1,"size":36260435,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmVcYit7yaLdxfaANZeawMSrH5teWBM6AhtFszZvuFC2ni"}],["0xfeef3d37a69ff35e4087736f06c9ee0c487ab4dd53e0c73fca12bb3cc1ed7f02",{"owner":442,"added_at":{"block":1126970,"time":1591294926000},"type_id":1,"size":440201919,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmadjBxxPasDupmedSKcxDJSYenuYMym6eqHRVdBpYcjbv"}],["0xff6e55a2e0b3332fae51ff7e32975a7d7ab5bce3f0bd1850ae00e08dac0fa751",{"owner":442,"added_at":{"block":1129846,"time":1591312386000},"type_id":1,"size":46196651,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmdCvapqKfoVAjNvpsnzecBug9y9LBikgyDYbUsKktuaRs"}],["0xffcb7d08e62559cb6624c2cd82cdccba811423d2b43522cef254a2312571fa77",{"owner":442,"added_at":{"block":1110975,"time":1591198638000},"type_id":1,"size":171629454,"liaison":0,"liaison_judgement":1,"ipfs_content_id":"QmP5U82G3zBS7Z2nY3M877knEHjMBqZw5v6Y2z6vpihFaG"}]]
  `
  const parsed = JSON.parse(exported)

  const sudoAddress = (await api.query.sudo.key()).toString()
  let sudo
  if (typeof window === 'undefined') {
    // In node, get the keyPair if the keyring was provided
    sudo = keyring.getPair(sudoAddress)
  } else {
    // Pioneer: let the UI Signer handle it
    sudo = sudoAddress
  }

  let nonce = (await api.query.system.account(sudoAddress)).nonce
  const max = api.consts.dataDirectory.maxObjectsPerInjection.toNumber()

  const preInjectionIds = await api.query.dataDirectory.knownContentIds()
  console.log(`Before injection there are ${preInjectionIds.length} known object ids`)

  // split injection into batches of max objects
  while (parsed.length) {
    const batch = parsed.splice(0, max)
    const objectsMap = api.createType('DataObjectsMap') // new joy.media.DataObjectsMap(api.registry)
    batch.forEach(([id, object]) => {
      objectsMap.set(api.createType('ContentId', id), api.createType('DataObject', object))
    })

    const injectTx = api.tx.dataDirectory.injectDataObjects(objectsMap)
    const sudoTx = api.tx.sudo.sudo(injectTx)
    console.log(`injecting ${batch.length} objects`)
    const signed = sudoTx.sign(sudo, { nonce })
    await signed.send()
    console.log(`nonce: ${nonce.toNumber()}, tx hash: ${signed.hash}`)
    nonce = nonce.addn(1)
  }
}

if (typeof module === 'undefined') {
  // Pioneer js-toolbox
  script({ api, hashing, keyring, types, util })
} else {
  // Node
  module.exports = script
}
