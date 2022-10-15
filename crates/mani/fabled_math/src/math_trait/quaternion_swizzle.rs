pub trait QuaternionSwizzles: Sized + Copy + Clone {
    fn iiii(self) -> Self;

    fn iiij(self) -> Self;

    fn iiik(self) -> Self;

    fn iiiw(self) -> Self;

    fn iiji(self) -> Self;

    fn iijj(self) -> Self;

    fn iijk(self) -> Self;

    fn iijw(self) -> Self;

    fn iiki(self) -> Self;

    fn iikj(self) -> Self;

    fn iikk(self) -> Self;

    fn iikw(self) -> Self;

    fn iiwi(self) -> Self;

    fn iiwj(self) -> Self;

    fn iiwk(self) -> Self;

    fn iiww(self) -> Self;

    fn ijii(self) -> Self;

    fn ijij(self) -> Self;

    fn ijik(self) -> Self;

    fn ijiw(self) -> Self;

    fn ijji(self) -> Self;

    fn ijjj(self) -> Self;

    fn ijjk(self) -> Self;

    fn ijjw(self) -> Self;

    fn ijki(self) -> Self;

    fn ijkj(self) -> Self;

    fn ijkk(self) -> Self;

    fn ijwi(self) -> Self;

    fn ijwj(self) -> Self;

    fn ijwk(self) -> Self;

    fn ijww(self) -> Self;

    fn ikii(self) -> Self;

    fn ikij(self) -> Self;

    fn ikik(self) -> Self;

    fn ikiw(self) -> Self;

    fn ikji(self) -> Self;

    fn ikjj(self) -> Self;

    fn ikjk(self) -> Self;

    fn ikjw(self) -> Self;

    fn ikki(self) -> Self;

    fn ikkj(self) -> Self;

    fn ikkk(self) -> Self;

    fn ikkw(self) -> Self;

    fn ikwi(self) -> Self;

    fn ikwj(self) -> Self;

    fn ikwk(self) -> Self;

    fn ikww(self) -> Self;

    fn iwii(self) -> Self;

    fn iwij(self) -> Self;

    fn iwik(self) -> Self;

    fn iwiw(self) -> Self;

    fn iwji(self) -> Self;

    fn iwjj(self) -> Self;

    fn iwjk(self) -> Self;

    fn iwjw(self) -> Self;

    fn iwki(self) -> Self;

    fn iwkj(self) -> Self;

    fn iwkk(self) -> Self;

    fn iwkw(self) -> Self;

    fn iwwi(self) -> Self;

    fn iwwj(self) -> Self;

    fn iwwk(self) -> Self;

    fn iwww(self) -> Self;

    fn jiii(self) -> Self;

    fn jiij(self) -> Self;

    fn jiik(self) -> Self;

    fn jiiw(self) -> Self;

    fn jiji(self) -> Self;

    fn jijj(self) -> Self;

    fn jijk(self) -> Self;

    fn jijw(self) -> Self;

    fn jiki(self) -> Self;

    fn jikj(self) -> Self;

    fn jikk(self) -> Self;

    fn jikw(self) -> Self;

    fn jiwi(self) -> Self;

    fn jiwj(self) -> Self;

    fn jiwk(self) -> Self;

    fn jiww(self) -> Self;

    fn jjii(self) -> Self;

    fn jjij(self) -> Self;

    fn jjik(self) -> Self;

    fn jjiw(self) -> Self;

    fn jjji(self) -> Self;

    fn jjjj(self) -> Self;

    fn jjjk(self) -> Self;

    fn jjjw(self) -> Self;

    fn jjki(self) -> Self;

    fn jjkj(self) -> Self;

    fn jjkk(self) -> Self;

    fn jjkw(self) -> Self;

    fn jjwi(self) -> Self;

    fn jjwj(self) -> Self;

    fn jjwk(self) -> Self;

    fn jjww(self) -> Self;

    fn jkii(self) -> Self;

    fn jkij(self) -> Self;

    fn jkik(self) -> Self;

    fn jkiw(self) -> Self;

    fn jkji(self) -> Self;

    fn jkjj(self) -> Self;

    fn jkjk(self) -> Self;

    fn jkjw(self) -> Self;

    fn jkki(self) -> Self;

    fn jkkj(self) -> Self;

    fn jkkk(self) -> Self;

    fn jkkw(self) -> Self;

    fn jkwi(self) -> Self;

    fn jkwj(self) -> Self;

    fn jkwk(self) -> Self;

    fn jkww(self) -> Self;

    fn jwii(self) -> Self;

    fn jwij(self) -> Self;

    fn jwik(self) -> Self;

    fn jwiw(self) -> Self;

    fn jwji(self) -> Self;

    fn jwjj(self) -> Self;

    fn jwjk(self) -> Self;

    fn jwjw(self) -> Self;

    fn jwki(self) -> Self;

    fn jwkj(self) -> Self;

    fn jwkk(self) -> Self;

    fn jwkw(self) -> Self;

    fn jwwi(self) -> Self;

    fn jwwj(self) -> Self;

    fn jwwk(self) -> Self;

    fn jwww(self) -> Self;

    fn kiii(self) -> Self;

    fn kiij(self) -> Self;

    fn kiik(self) -> Self;

    fn kiiw(self) -> Self;

    fn kiji(self) -> Self;

    fn kijj(self) -> Self;

    fn kijk(self) -> Self;

    fn kijw(self) -> Self;

    fn kiki(self) -> Self;

    fn kikj(self) -> Self;

    fn kikk(self) -> Self;

    fn kikw(self) -> Self;

    fn kiwi(self) -> Self;

    fn kiwj(self) -> Self;

    fn kiwk(self) -> Self;

    fn kiww(self) -> Self;

    fn kjii(self) -> Self;

    fn kjij(self) -> Self;

    fn kjik(self) -> Self;

    fn kjiw(self) -> Self;

    fn kjji(self) -> Self;

    fn kjjj(self) -> Self;

    fn kjjk(self) -> Self;

    fn kjjw(self) -> Self;

    fn kjki(self) -> Self;

    fn kjkj(self) -> Self;

    fn kjkk(self) -> Self;

    fn kjkw(self) -> Self;

    fn kjwi(self) -> Self;

    fn kjwj(self) -> Self;

    fn kjwk(self) -> Self;

    fn kjww(self) -> Self;

    fn kkii(self) -> Self;

    fn kkij(self) -> Self;

    fn kkik(self) -> Self;

    fn kkiw(self) -> Self;

    fn kkji(self) -> Self;

    fn kkjj(self) -> Self;

    fn kkjk(self) -> Self;

    fn kkjw(self) -> Self;

    fn kkki(self) -> Self;

    fn kkkj(self) -> Self;

    fn kkkk(self) -> Self;

    fn kkkw(self) -> Self;

    fn kkwi(self) -> Self;

    fn kkwj(self) -> Self;

    fn kkwk(self) -> Self;

    fn kkww(self) -> Self;

    fn kwii(self) -> Self;

    fn kwij(self) -> Self;

    fn kwik(self) -> Self;

    fn kwiw(self) -> Self;

    fn kwji(self) -> Self;

    fn kwjj(self) -> Self;

    fn kwjk(self) -> Self;

    fn kwjw(self) -> Self;

    fn kwki(self) -> Self;

    fn kwkj(self) -> Self;

    fn kwkk(self) -> Self;

    fn kwkw(self) -> Self;

    fn kwwi(self) -> Self;

    fn kwwj(self) -> Self;

    fn kwwk(self) -> Self;

    fn kwww(self) -> Self;

    fn wiii(self) -> Self;

    fn wiij(self) -> Self;

    fn wiik(self) -> Self;

    fn wiiw(self) -> Self;

    fn wiji(self) -> Self;

    fn wijj(self) -> Self;

    fn wijk(self) -> Self;

    fn wijw(self) -> Self;

    fn wiki(self) -> Self;

    fn wikj(self) -> Self;

    fn wikk(self) -> Self;

    fn wikw(self) -> Self;

    fn wiwi(self) -> Self;

    fn wiwj(self) -> Self;

    fn wiwk(self) -> Self;

    fn wiww(self) -> Self;

    fn wjii(self) -> Self;

    fn wjij(self) -> Self;

    fn wjik(self) -> Self;

    fn wjiw(self) -> Self;

    fn wjji(self) -> Self;

    fn wjjj(self) -> Self;

    fn wjjk(self) -> Self;

    fn wjjw(self) -> Self;

    fn wjki(self) -> Self;

    fn wjkj(self) -> Self;

    fn wjkk(self) -> Self;

    fn wjkw(self) -> Self;

    fn wjwi(self) -> Self;

    fn wjwj(self) -> Self;

    fn wjwk(self) -> Self;

    fn wjww(self) -> Self;

    fn wkii(self) -> Self;

    fn wkij(self) -> Self;

    fn wkik(self) -> Self;

    fn wkiw(self) -> Self;

    fn wkji(self) -> Self;

    fn wkjj(self) -> Self;

    fn wkjk(self) -> Self;

    fn wkjw(self) -> Self;

    fn wkki(self) -> Self;

    fn wkkj(self) -> Self;

    fn wkkk(self) -> Self;

    fn wkkw(self) -> Self;

    fn wkwi(self) -> Self;

    fn wkwj(self) -> Self;

    fn wkwk(self) -> Self;

    fn wkww(self) -> Self;

    fn wwii(self) -> Self;

    fn wwij(self) -> Self;

    fn wwik(self) -> Self;

    fn wwiw(self) -> Self;

    fn wwji(self) -> Self;

    fn wwjj(self) -> Self;

    fn wwjk(self) -> Self;

    fn wwjw(self) -> Self;

    fn wwki(self) -> Self;

    fn wwkj(self) -> Self;

    fn wwkk(self) -> Self;

    fn wwkw(self) -> Self;

    fn wwwi(self) -> Self;

    fn wwwj(self) -> Self;

    fn wwwk(self) -> Self;

    fn wwww(self) -> Self;
}
