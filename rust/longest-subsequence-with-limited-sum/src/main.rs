impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        // nums.push(1000);
        nums.sort_unstable();
        let mut sum: u64 = 0;
        let sums: Vec<u64> = nums
            .iter()
            .map(|&x| {
                sum += x as u64;
                sum
            })
            .collect();
        println!("sums: {:?}", sums);
        queries
            .iter()
            .map(|q| {
                (match sums.binary_search(&(*q as u64)) {
                    Ok(x) => x + 1,
                    Err(x) => x,
                }) as i32
            })
            .collect()
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21])
    );
    println!("{:?}", Solution::answer_queries(vec![3, 4, 5, 2], vec![1]));
    println!("{:?}", Solution::answer_queries(vec![1], vec![1]));
    println!("{:?}", Solution::answer_queries(vec![2], vec![1]));
    println!(
        "{:?}",
        Solution::answer_queries(vec![1, 888], vec![887, 888, 889])
    );
    /*println!(
        "{:?}",
        Solution::answer_queries(
            vec![
                839222, 106083, 401656, 548848, 863647, 497103, 372194, 488055, 802430, 391568,
                806603, 175422, 991023, 53036, 231010, 584357, 605689, 815630, 561667, 928393,
                50216, 438027, 22161, 301775, 165266, 952109, 500083, 884525, 814381, 344273,
                670860, 854766, 547708, 11573, 810568, 254394, 388012, 383459, 444487, 924919,
                616503, 224784, 774976, 779655, 388321, 493783, 158742, 741575, 34900, 360744,
                90976, 312771, 503819, 655884, 232973, 428029, 273007, 495576, 608967, 514136,
                274733, 60576, 313024, 580612, 387903, 116604, 948982, 846827, 339736, 90459,
                169146, 25516, 779907, 621900, 597952, 106316, 784322, 575136, 880976, 885690,
                471489, 576709, 720512, 442681, 801562, 9285, 285698, 45340, 966182, 809426,
                428208, 956318, 601916, 58547, 131129, 930050, 230321, 606224, 857430, 337810,
                808227, 591681, 392074, 304362, 771806, 883430, 166221, 918846, 655583, 151179,
                952337, 362834, 657444, 441507, 237772, 937128, 511406, 886435, 692789, 232527,
                633314, 974346, 621791, 902129, 374634, 25112, 42382, 58482, 76609, 278788, 177441,
                142558, 666257, 902562, 357486, 402141, 440657, 138035, 765222, 892747, 692861,
                876317, 254808, 314708, 180331, 189927, 874946, 509350, 80264, 740995, 375906,
                747845, 289483, 215353, 194588, 109687, 412159, 960209, 776551, 270604, 18876,
                690813, 860163, 858915, 307110, 644417, 228873, 716140, 924434, 532047, 737500,
                261795, 220812, 658324, 868025, 320898, 308122, 798379, 840856, 177767, 820190,
                576350, 619098, 246090, 329148, 58688, 963391, 576025, 694804, 428729, 432685,
                56356, 951094, 263679, 230895, 644223, 412520, 201990, 29032, 743520, 510931,
                867153, 97598, 670403, 607150, 528175, 363443, 54165, 62996, 524032, 469004,
                498473, 549661, 860001, 700425, 628043, 815956, 779957, 23780, 936291, 84790,
                899722, 977845, 165645, 730783, 416860, 963247, 83995, 586411, 998372, 991007,
                438680, 992826, 977564, 86140, 555227, 810164, 62667, 993793, 679450, 630940,
                73626, 359796, 320952, 381800, 349806, 878585, 329849, 187767, 454131, 987196,
                42359, 659610, 684973, 127867, 907227, 664966, 223568, 993721, 192514, 365829,
                269787, 258101, 27290, 925920, 196156, 583978, 73239, 635173, 137922, 364195,
                831110, 275336, 546200, 136944, 650273, 271526, 14372, 784166, 992560, 117812,
                680681, 300090, 392704, 756799, 201483, 339390, 846800, 708042, 734316, 500642,
                798234, 29612, 942028, 895306, 771047, 486176, 853251, 951969, 501417, 153553,
                723577, 719708, 729144, 850669, 453071, 196138, 784169, 659018, 979694, 39126,
                366644, 165890, 982296, 915605, 875243, 101386, 618024, 286302, 136391, 837974,
                475323, 595747, 354896, 899486, 719496, 18524, 471038, 997983, 648655, 500612,
                186959, 579755, 292767, 394578, 361146, 658220, 548434, 557732, 211462, 603619,
                814704, 578305, 103182, 549715, 209353, 742163, 117231, 489704, 908037, 212000,
                317356, 248193, 876495, 790001, 445962, 749011, 612448, 522511, 351833, 685690,
                244353, 255700, 686196, 230949, 175850, 528753, 97429, 420551, 686742, 44264,
                661749, 822693, 766473, 216499, 51813, 67920, 448166, 514168, 927259, 197947,
                677367, 423792, 409621, 594063, 977018, 177837, 610268, 642293, 305764, 813897,
                265079, 209199, 154407, 876101, 15446, 945590, 697232, 583619, 297795, 916632,
                370287, 964828, 968220, 500528, 920719, 615970, 945724, 814095, 792336, 403186,
                494035, 503411, 841430, 620770, 451810, 669474, 634506, 775437, 78181, 756267,
                189241, 822864, 463228, 766698, 29899, 657455, 693573, 595078, 917418, 150863,
                710224, 445382, 73247, 857465, 94519, 994159, 631642, 233771, 598310, 459757,
                432834, 447089, 216235, 669868, 942652, 342480, 94997, 861600, 960595, 346108,
                422660, 797040, 412175, 799594, 426922, 781209, 750457, 711073, 23739, 448131,
                546504, 177071, 12605, 498010, 357003, 153795, 755352, 346042, 785206, 402073,
                32594, 47326, 173213, 925257, 806638, 569920, 185895, 19336, 358383, 986923, 73139,
                150676, 624631, 49793, 48305, 502578, 586217, 405732, 773422, 497255, 549064,
                603137, 575573, 877403, 241265, 94426, 489334, 176428, 836101, 750989, 869314,
                196011, 753389, 869664, 458954, 628048, 985575, 359106, 925387, 353882, 761756,
                537944, 255722, 365338, 869223, 798822, 125631, 348535, 580472, 462495, 169468,
                211920, 739116, 672420, 169135, 247194, 615857, 617450, 146704, 515192, 575187,
                862947, 873940, 583197, 69607, 127694, 922889, 374634, 763367, 354620, 472391,
                461186, 40593, 564694, 987173, 188143, 670597, 646279, 64832, 297662, 439078,
                853974, 81041, 839138, 671719, 818566, 258400, 210926, 449400, 853702, 854101,
                809344, 426497, 445416, 587983, 334078, 63258, 992391, 410441, 374951, 877787,
                510307, 518687, 699910, 718163, 142523, 772493
            ],
            vec![
                846459, 10126, 833867, 296428, 382961, 130370, 575763, 121438, 524291, 688829,
                582585, 965444, 684665, 481768, 949946, 564473, 613411, 843484, 687413, 367322,
                198303, 462891, 910176, 627986, 62769, 38883, 314411, 66616, 191743, 705571,
                998065, 608461, 245256, 362657, 418077, 655146, 458442, 4202, 55582, 936937,
                979329, 19430, 629316, 565900, 336901, 672341, 161582, 487883, 451039, 715512,
                56618, 380949, 705130, 299977, 949580, 122037, 567174, 375767, 574704, 818399,
                993768, 416127, 855394, 161334, 9095, 859325, 830300, 282567, 765570, 455128,
                252195, 883841, 257830, 274731, 396116, 203561, 763538, 356157, 80001, 476418,
                118107, 597811, 343386, 321419, 905084, 174833, 852620, 312534, 363496, 752986,
                144691, 139191, 483463, 334610, 473169, 835540, 112300, 824608, 464217, 113542,
                379504, 32053, 740628, 536, 364391, 589348, 282616, 783295, 319722, 506583, 809862,
                533466, 489129, 925356, 523666, 484722, 711314, 902409, 386348, 115799, 946649,
                637291, 243269, 680731, 593541, 34843, 565451, 319349, 432389, 39761, 826774,
                973762, 835438, 671529, 675097, 976187, 162768, 836332, 219276, 397352, 36448,
                442740, 146420, 150571, 680125, 662143, 772252, 419095, 775022, 511261, 866441,
                706704, 164169, 201673, 761320, 872495, 827355, 695991, 311192, 202447, 541369,
                163268, 941450, 983115, 547894, 16673, 404929, 44453, 879530, 895014, 19577,
                344265, 997012, 37742, 364318, 818427, 650687, 673732, 317073, 900032, 985707,
                434103, 808447, 140343, 399111, 473839, 683770, 821176, 839986, 416398, 763957,
                408954, 852829, 769553, 79442, 874663, 69389, 205582, 437750, 308717, 129945,
                135874, 323343, 225676, 604605, 579598, 329005, 951396, 254911, 103431, 769057,
                388351, 919586, 779660, 706151, 597456, 834496, 487947, 562633, 351861, 960906,
                769809, 231568, 295793, 70340, 890490, 312856, 128017, 351814, 394036, 326296,
                720148, 310759, 576206, 275783, 568562, 347277, 445900, 917660, 974923, 605155,
                53237, 85593, 297661, 236076, 152277, 323877, 531055, 280995, 122642, 853355,
                123466, 694444, 77814, 959420, 900454, 528867, 599893, 502413, 473648, 675618,
                993018, 872505, 589679, 607543, 334423, 209009, 499111, 5677, 379981, 597236,
                510546, 372353, 80124, 68597, 710708, 875225, 432865, 980553, 416012, 414767,
                175914, 110671, 905400, 293904, 726862, 879196, 838481, 621832, 536328, 645831,
                222022, 136845, 508453, 616885, 331991, 282405, 319719, 260714, 792064, 908371,
                178337, 713200, 891596, 877868, 806093, 515415, 809709, 856269, 505976, 820751,
                909138, 404070, 803187, 936552, 946027, 750662, 426172, 129814, 165373, 889780,
                946596, 606085, 227876, 991007, 790704, 648288, 977478, 947474, 759878, 273476,
                888407, 357504, 864433, 113088, 980657, 619333, 784536, 726369, 805846, 179358,
                400821, 15243, 459993, 352186, 743305, 200374, 328854, 614152, 424497, 301324,
                231626, 453845, 282835, 100808, 63031, 911057, 816497, 966147, 59004, 185949,
                227426, 379403, 134396, 439954, 739564, 16890, 183994, 255470, 220046, 399814,
                649470, 528718, 263957, 658612, 965273, 380244, 455235, 629214, 934567, 224704,
                870681, 322855, 694292, 634801, 518669, 681972, 140116, 509129, 694752, 296530,
                380698, 914529, 130202, 10700, 475561, 633086, 794727, 952385, 590377, 82792,
                902653, 352482, 514478, 363798, 795059, 647827, 912411, 851953, 517079, 368085,
                342707, 907336, 469780, 376334, 685886, 211755, 199876, 802118, 608912, 667376,
                711907, 340749, 168395, 607744, 379165, 290101, 685331, 764813, 726135, 132798,
                393821, 952581, 651412, 82762, 244233, 853148, 167482, 236706, 994408, 724152,
                501701, 416380, 176209, 641777, 405712, 615237, 812653, 681529, 231392, 799378,
                732271, 626540, 98199, 485376, 887982, 88988, 679058, 172212, 351761, 500319,
                450324, 947035, 19065, 231039, 485253, 894363, 354001, 997670, 630896, 945327,
                88910, 231984, 860488, 701630, 245441, 827427, 653195, 35811, 73316, 390601,
                286843, 258416, 487397, 810495, 442337, 777600, 127470, 592538, 34137, 420658,
                202639, 138756, 432459, 980227, 145447, 431118, 821693, 53902, 116840, 538430,
                18356, 901620, 829408, 701723, 48783, 768870, 622661, 850807, 210460, 147096,
                470475, 872144, 216947, 459643, 804287, 425825, 949728, 275838, 173089, 767297,
                331274, 951627, 765374, 501389, 87937, 91774, 39333, 355584, 794047, 256658
            ]
        )
    ); */
}
