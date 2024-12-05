#include "fheco/fheco.hpp"

using namespace std;
using namespace fheco;
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector> 
/********************/
void fhe_vectorized(int slot_count){
  Ciphertext c1("c1"); 
  Ciphertext c2("c2");
  Ciphertext slot_wise_diff = (c1 - c2) * (c1 - c2);
  Ciphertext sum = SumVec(slot_wise_diff,slot_count);
  sum.set_output("result"); 
}
/***************************/
// void fhe(int slot_count)
// {
//   size_t size = slot_count;
//   std::vector<Ciphertext> v1(size);
//   std::vector<Ciphertext> v2(size);
//   //std::vector<Ciphertext> output(size);
//   Ciphertext output = Ciphertext(Plaintext(0));
//   for (int i = 0; i < size; i++)
//   {
//     v1[i] = Ciphertext("c1_" + std::to_string(i));
//   }
//   for (int i = 0; i < size; i++)
//   { 
//     v2[i] = Ciphertext("c2_" + std::to_string(i));
//     if(i==0){
//         output = (v2[i] - v1[i]) * (v2[i] - v1[i]); 
//     }else{
//       output += (v2[i] - v1[i]) * (v2[i] - v1[i]);
//     }
//   }
//   output.set_output("result");
// }

void fhe(int slot_count)
{

    Ciphertext output;

    output = (((((((((((Ciphertext("v_973") * Ciphertext("v_187")) * (Ciphertext("v_749") * Ciphertext("v_798"))) * ((Ciphertext("v_613") * Ciphertext("v_611")) * (Ciphertext("v_159") * Ciphertext("v_456")))) * (((Ciphertext("v_159") * Ciphertext("v_102")) * (Ciphertext("v_59") * Ciphertext("v_470"))) * ((Ciphertext("v_886") * Ciphertext("v_341")) * (Ciphertext("v_615") * Ciphertext("v_146"))))) * ((((Ciphertext("v_725") * Ciphertext("v_666")) * (Ciphertext("v_21") * Ciphertext("v_57"))) * ((Ciphertext("v_993") * Ciphertext("v_739")) * (Ciphertext("v_852") * Ciphertext("v_961")))) * (((Ciphertext("v_217") * Ciphertext("v_0")) * (Ciphertext("v_186") * Ciphertext("v_1016"))) * ((Ciphertext("v_187") * Ciphertext("v_632")) * (Ciphertext("v_311") * Ciphertext("v_626")))))) * (((((Ciphertext("v_537") * Ciphertext("v_7")) * (Ciphertext("v_442") * Ciphertext("v_23"))) * ((Ciphertext("v_298") * Ciphertext("v_537")) * (Ciphertext("v_626") * Ciphertext("v_409")))) * (((Ciphertext("v_142") * Ciphertext("v_47")) * (Ciphertext("v_299") * Ciphertext("v_997"))) * ((Ciphertext("v_375") * Ciphertext("v_238")) * (Ciphertext("v_467") * Ciphertext("v_92"))))) * ((((Ciphertext("v_804") * Ciphertext("v_633")) * (Ciphertext("v_204") * Ciphertext("v_391"))) * ((Ciphertext("v_526") * Ciphertext("v_1006")) * (Ciphertext("v_606") * Ciphertext("v_477")))) * (((Ciphertext("v_47") * Ciphertext("v_880")) * (Ciphertext("v_622") * Ciphertext("v_696"))) * ((Ciphertext("v_174") * Ciphertext("v_461")) * (Ciphertext("v_66") * Ciphertext("v_13"))))))) * ((((((Ciphertext("v_971") * Ciphertext("v_964")) * (Ciphertext("v_988") * Ciphertext("v_576"))) * ((Ciphertext("v_827") * Ciphertext("v_394")) * (Ciphertext("v_311") * Ciphertext("v_16")))) * (((Ciphertext("v_100") * Ciphertext("v_236")) * (Ciphertext("v_700") * Ciphertext("v_246"))) * ((Ciphertext("v_450") * Ciphertext("v_699")) * (Ciphertext("v_124") * Ciphertext("v_624"))))) * ((((Ciphertext("v_507") * Ciphertext("v_853")) * (Ciphertext("v_35") * Ciphertext("v_177"))) * ((Ciphertext("v_931") * Ciphertext("v_400")) * (Ciphertext("v_264") * Ciphertext("v_186")))) * (((Ciphertext("v_678") * Ciphertext("v_773")) * (Ciphertext("v_319") * Ciphertext("v_435"))) * ((Ciphertext("v_532") * Ciphertext("v_212")) * (Ciphertext("v_559") * Ciphertext("v_581")))))) * (((((Ciphertext("v_189") * Ciphertext("v_32")) * (Ciphertext("v_992") * Ciphertext("v_862"))) * ((Ciphertext("v_793") * Ciphertext("v_460")) * (Ciphertext("v_962") * Ciphertext("v_404")))) * (((Ciphertext("v_916") * Ciphertext("v_948")) * (Ciphertext("v_612") * Ciphertext("v_744"))) * ((Ciphertext("v_943") * Ciphertext("v_334")) * (Ciphertext("v_90") * Ciphertext("v_584"))))) * ((((Ciphertext("v_200") * Ciphertext("v_533")) * (Ciphertext("v_46") * Ciphertext("v_984"))) * ((Ciphertext("v_333") * Ciphertext("v_864")) * (Ciphertext("v_398") * Ciphertext("v_765")))) * (((Ciphertext("v_277") * Ciphertext("v_552")) * (Ciphertext("v_848") * Ciphertext("v_600"))) * ((Ciphertext("v_365") * Ciphertext("v_988")) * (Ciphertext("v_287") * Ciphertext("v_621")))))))) * (((((((Ciphertext("v_555") * Ciphertext("v_282")) * (Ciphertext("v_144") * Ciphertext("v_303"))) * ((Ciphertext("v_821") * Ciphertext("v_169")) * (Ciphertext("v_76") * Ciphertext("v_16")))) * (((Ciphertext("v_1010") * Ciphertext("v_433")) * (Ciphertext("v_790") * Ciphertext("v_404"))) * ((Ciphertext("v_203") * Ciphertext("v_300")) * (Ciphertext("v_5") * Ciphertext("v_14"))))) * ((((Ciphertext("v_835") * Ciphertext("v_203")) * (Ciphertext("v_723") * Ciphertext("v_728"))) * ((Ciphertext("v_746") * Ciphertext("v_809")) * (Ciphertext("v_789") * Ciphertext("v_620")))) * (((Ciphertext("v_75") * Ciphertext("v_948")) * (Ciphertext("v_367") * Ciphertext("v_666"))) * ((Ciphertext("v_118") * Ciphertext("v_936")) * (Ciphertext("v_883") * Ciphertext("v_870")))))) * (((((Ciphertext("v_638") * Ciphertext("v_460")) * (Ciphertext("v_338") * Ciphertext("v_97"))) * ((Ciphertext("v_65") * Ciphertext("v_379")) * (Ciphertext("v_318") * Ciphertext("v_684")))) * (((Ciphertext("v_332") * Ciphertext("v_681")) * (Ciphertext("v_747") * Ciphertext("v_605"))) * ((Ciphertext("v_652") * Ciphertext("v_281")) * (Ciphertext("v_908") * Ciphertext("v_574"))))) * ((((Ciphertext("v_483") * Ciphertext("v_392")) * (Ciphertext("v_122") * Ciphertext("v_995"))) * ((Ciphertext("v_730") * Ciphertext("v_869")) * (Ciphertext("v_779") * Ciphertext("v_739")))) * (((Ciphertext("v_574") * Ciphertext("v_241")) * (Ciphertext("v_789") * Ciphertext("v_262"))) * ((Ciphertext("v_505") * Ciphertext("v_41")) * (Ciphertext("v_535") * Ciphertext("v_727"))))))) * ((((((Ciphertext("v_437") * Ciphertext("v_113")) * (Ciphertext("v_26") * Ciphertext("v_449"))) * ((Ciphertext("v_110") * Ciphertext("v_206")) * (Ciphertext("v_32") * Ciphertext("v_917")))) * (((Ciphertext("v_651") * Ciphertext("v_486")) * (Ciphertext("v_321") * Ciphertext("v_576"))) * ((Ciphertext("v_520") * Ciphertext("v_712")) * (Ciphertext("v_929") * Ciphertext("v_142"))))) * ((((Ciphertext("v_255") * Ciphertext("v_618")) * (Ciphertext("v_420") * Ciphertext("v_552"))) * ((Ciphertext("v_773") * Ciphertext("v_207")) * (Ciphertext("v_234") * Ciphertext("v_965")))) * (((Ciphertext("v_78") * Ciphertext("v_613")) * (Ciphertext("v_296") * Ciphertext("v_711"))) * ((Ciphertext("v_165") * Ciphertext("v_901")) * (Ciphertext("v_952") * Ciphertext("v_639")))))) * (((((Ciphertext("v_827") * Ciphertext("v_302")) * (Ciphertext("v_648") * Ciphertext("v_108"))) * ((Ciphertext("v_892") * Ciphertext("v_467")) * (Ciphertext("v_822") * Ciphertext("v_223")))) * (((Ciphertext("v_191") * Ciphertext("v_426")) * (Ciphertext("v_913") * Ciphertext("v_904"))) * ((Ciphertext("v_552") * Ciphertext("v_332")) * (Ciphertext("v_826") * Ciphertext("v_125"))))) * ((((Ciphertext("v_917") * Ciphertext("v_364")) * (Ciphertext("v_325") * Ciphertext("v_928"))) * ((Ciphertext("v_112") * Ciphertext("v_278")) * (Ciphertext("v_233") * Ciphertext("v_663")))) * (((Ciphertext("v_437") * Ciphertext("v_0")) * (Ciphertext("v_837") * Ciphertext("v_361"))) * ((Ciphertext("v_881") * Ciphertext("v_312")) * (Ciphertext("v_7") * Ciphertext("v_168"))))))))) + ((((((((Ciphertext("v_523") * Ciphertext("v_546")) * (Ciphertext("v_427") * Ciphertext("v_496"))) * ((Ciphertext("v_227") * Ciphertext("v_709")) * (Ciphertext("v_122") * Ciphertext("v_275")))) * (((Ciphertext("v_345") * Ciphertext("v_249")) * (Ciphertext("v_965") * Ciphertext("v_172"))) * ((Ciphertext("v_330") * Ciphertext("v_224")) * (Ciphertext("v_531") * Ciphertext("v_571"))))) * ((((Ciphertext("v_719") * Ciphertext("v_413")) * (Ciphertext("v_372") * Ciphertext("v_66"))) * ((Ciphertext("v_995") * Ciphertext("v_260")) * (Ciphertext("v_985") * Ciphertext("v_252")))) * (((Ciphertext("v_257") * Ciphertext("v_713")) * (Ciphertext("v_509") * Ciphertext("v_729"))) * ((Ciphertext("v_308") * Ciphertext("v_151")) * (Ciphertext("v_291") * Ciphertext("v_1021")))))) * (((((Ciphertext("v_37") * Ciphertext("v_273")) * (Ciphertext("v_624") * Ciphertext("v_1000"))) * ((Ciphertext("v_514") * Ciphertext("v_420")) * (Ciphertext("v_52") * Ciphertext("v_33")))) * (((Ciphertext("v_285") * Ciphertext("v_353")) * (Ciphertext("v_930") * Ciphertext("v_649"))) * ((Ciphertext("v_245") * Ciphertext("v_697")) * (Ciphertext("v_148") * Ciphertext("v_543"))))) * ((((Ciphertext("v_501") * Ciphertext("v_458")) * (Ciphertext("v_1009") * Ciphertext("v_566"))) * ((Ciphertext("v_247") * Ciphertext("v_606")) * (Ciphertext("v_688") * Ciphertext("v_82")))) * (((Ciphertext("v_779") * Ciphertext("v_378")) * (Ciphertext("v_243") * Ciphertext("v_247"))) * ((Ciphertext("v_745") * Ciphertext("v_822")) * (Ciphertext("v_376") * Ciphertext("v_481"))))))) * ((((((Ciphertext("v_647") * Ciphertext("v_1007")) * (Ciphertext("v_648") * Ciphertext("v_408"))) * ((Ciphertext("v_548") * Ciphertext("v_836")) * (Ciphertext("v_92") * Ciphertext("v_817")))) * (((Ciphertext("v_855") * Ciphertext("v_154")) * (Ciphertext("v_328") * Ciphertext("v_520"))) * ((Ciphertext("v_190") * Ciphertext("v_712")) * (Ciphertext("v_41") * Ciphertext("v_878"))))) * ((((Ciphertext("v_605") * Ciphertext("v_333")) * (Ciphertext("v_693") * Ciphertext("v_225"))) * ((Ciphertext("v_16") * Ciphertext("v_728")) * (Ciphertext("v_524") * Ciphertext("v_828")))) * (((Ciphertext("v_231") * Ciphertext("v_357")) * (Ciphertext("v_660") * Ciphertext("v_98"))) * ((Ciphertext("v_178") * Ciphertext("v_963")) * (Ciphertext("v_707") * Ciphertext("v_407")))))) * (((((Ciphertext("v_396") * Ciphertext("v_530")) * (Ciphertext("v_959") * Ciphertext("v_857"))) * ((Ciphertext("v_140") * Ciphertext("v_691")) * (Ciphertext("v_349") * Ciphertext("v_752")))) * (((Ciphertext("v_116") * Ciphertext("v_214")) * (Ciphertext("v_946") * Ciphertext("v_554"))) * ((Ciphertext("v_898") * Ciphertext("v_712")) * (Ciphertext("v_264") * Ciphertext("v_234"))))) * ((((Ciphertext("v_675") * Ciphertext("v_179")) * (Ciphertext("v_836") * Ciphertext("v_1005"))) * ((Ciphertext("v_568") * Ciphertext("v_529")) * (Ciphertext("v_542") * Ciphertext("v_267")))) * (((Ciphertext("v_247") * Ciphertext("v_1020")) * (Ciphertext("v_95") * Ciphertext("v_988"))) * ((Ciphertext("v_918") * Ciphertext("v_571")) * (Ciphertext("v_922") * Ciphertext("v_903")))))))) * (((((((Ciphertext("v_648") * Ciphertext("v_193")) * (Ciphertext("v_347") * Ciphertext("v_285"))) * ((Ciphertext("v_357") * Ciphertext("v_717")) * (Ciphertext("v_743") * Ciphertext("v_866")))) * (((Ciphertext("v_918") * Ciphertext("v_876")) * (Ciphertext("v_908") * Ciphertext("v_414"))) * ((Ciphertext("v_798") * Ciphertext("v_909")) * (Ciphertext("v_657") * Ciphertext("v_871"))))) * ((((Ciphertext("v_86") * Ciphertext("v_958")) * (Ciphertext("v_165") * Ciphertext("v_804"))) * ((Ciphertext("v_920") * Ciphertext("v_685")) * (Ciphertext("v_620") * Ciphertext("v_594")))) * (((Ciphertext("v_9") * Ciphertext("v_381")) * (Ciphertext("v_103") * Ciphertext("v_962"))) * ((Ciphertext("v_679") * Ciphertext("v_997")) * (Ciphertext("v_5") * Ciphertext("v_290")))))) * (((((Ciphertext("v_164") * Ciphertext("v_312")) * (Ciphertext("v_561") * Ciphertext("v_497"))) * ((Ciphertext("v_708") * Ciphertext("v_459")) * (Ciphertext("v_667") * Ciphertext("v_1018")))) * (((Ciphertext("v_229") * Ciphertext("v_180")) * (Ciphertext("v_729") * Ciphertext("v_18"))) * ((Ciphertext("v_242") * Ciphertext("v_505")) * (Ciphertext("v_333") * Ciphertext("v_183"))))) * ((((Ciphertext("v_764") * Ciphertext("v_375")) * (Ciphertext("v_665") * Ciphertext("v_762"))) * ((Ciphertext("v_869") * Ciphertext("v_738")) * (Ciphertext("v_673") * Ciphertext("v_315")))) * (((Ciphertext("v_581") * Ciphertext("v_555")) * (Ciphertext("v_95") * Ciphertext("v_521"))) * ((Ciphertext("v_376") * Ciphertext("v_651")) * (Ciphertext("v_271") * Ciphertext("v_256"))))))) * ((((((Ciphertext("v_249") * Ciphertext("v_604")) * (Ciphertext("v_996") * Ciphertext("v_1002"))) * ((Ciphertext("v_402") * Ciphertext("v_498")) * (Ciphertext("v_913") * Ciphertext("v_927")))) * (((Ciphertext("v_646") * Ciphertext("v_444")) * (Ciphertext("v_813") * Ciphertext("v_358"))) * ((Ciphertext("v_514") * Ciphertext("v_660")) * (Ciphertext("v_590") * Ciphertext("v_684"))))) * ((((Ciphertext("v_504") * Ciphertext("v_884")) * (Ciphertext("v_199") * Ciphertext("v_235"))) * ((Ciphertext("v_739") * Ciphertext("v_511")) * (Ciphertext("v_287") * Ciphertext("v_585")))) * (((Ciphertext("v_24") * Ciphertext("v_786")) * (Ciphertext("v_660") * Ciphertext("v_44"))) * ((Ciphertext("v_181") * Ciphertext("v_1018")) * (Ciphertext("v_963") * Ciphertext("v_481")))))) * (((((Ciphertext("v_976") * Ciphertext("v_286")) * (Ciphertext("v_936") * Ciphertext("v_904"))) * ((Ciphertext("v_379") * Ciphertext("v_765")) * (Ciphertext("v_15") * Ciphertext("v_975")))) * (((Ciphertext("v_950") * Ciphertext("v_338")) * (Ciphertext("v_438") * Ciphertext("v_566"))) * ((Ciphertext("v_989") * Ciphertext("v_586")) * (Ciphertext("v_986") * Ciphertext("v_1003"))))) * ((((Ciphertext("v_873") * Ciphertext("v_77")) * (Ciphertext("v_301") * Ciphertext("v_313"))) * ((Ciphertext("v_394") * Ciphertext("v_195")) * (Ciphertext("v_871") * Ciphertext("v_274")))) * (((Ciphertext("v_324") * Ciphertext("v_496")) * (Ciphertext("v_173") * Ciphertext("v_381"))) * ((Ciphertext("v_570") * Ciphertext("v_404")) * (Ciphertext("v_958") * Ciphertext("v_864")))))))))) * (((((((((Ciphertext("v_952") * Ciphertext("v_583")) * (Ciphertext("v_72") * Ciphertext("v_99"))) * ((Ciphertext("v_213") * Ciphertext("v_629")) * (Ciphertext("v_687") * Ciphertext("v_1013")))) * (((Ciphertext("v_367") * Ciphertext("v_143")) * (Ciphertext("v_260") * Ciphertext("v_530"))) * ((Ciphertext("v_302") * Ciphertext("v_898")) * (Ciphertext("v_330") * Ciphertext("v_758"))))) * ((((Ciphertext("v_869") * Ciphertext("v_713")) * (Ciphertext("v_139") * Ciphertext("v_719"))) * ((Ciphertext("v_725") * Ciphertext("v_368")) * (Ciphertext("v_566") * Ciphertext("v_300")))) * (((Ciphertext("v_303") * Ciphertext("v_828")) * (Ciphertext("v_429") * Ciphertext("v_829"))) * ((Ciphertext("v_262") * Ciphertext("v_887")) * (Ciphertext("v_626") * Ciphertext("v_935")))))) * (((((Ciphertext("v_83") * Ciphertext("v_523")) * (Ciphertext("v_5") * Ciphertext("v_513"))) * ((Ciphertext("v_642") * Ciphertext("v_817")) * (Ciphertext("v_198") * Ciphertext("v_665")))) * (((Ciphertext("v_72") * Ciphertext("v_718")) * (Ciphertext("v_406") * Ciphertext("v_814"))) * ((Ciphertext("v_51") * Ciphertext("v_911")) * (Ciphertext("v_907") * Ciphertext("v_346"))))) * ((((Ciphertext("v_28") * Ciphertext("v_384")) * (Ciphertext("v_592") * Ciphertext("v_96"))) * ((Ciphertext("v_448") * Ciphertext("v_592")) * (Ciphertext("v_688") * Ciphertext("v_36")))) * (((Ciphertext("v_336") * Ciphertext("v_476")) * (Ciphertext("v_158") * Ciphertext("v_555"))) * ((Ciphertext("v_1005") * Ciphertext("v_293")) * (Ciphertext("v_859") * Ciphertext("v_605"))))))) * ((((((Ciphertext("v_881") * Ciphertext("v_31")) * (Ciphertext("v_256") * Ciphertext("v_38"))) * ((Ciphertext("v_39") * Ciphertext("v_842")) * (Ciphertext("v_310") * Ciphertext("v_368")))) * (((Ciphertext("v_549") * Ciphertext("v_130")) * (Ciphertext("v_334") * Ciphertext("v_534"))) * ((Ciphertext("v_847") * Ciphertext("v_788")) * (Ciphertext("v_278") * Ciphertext("v_221"))))) * ((((Ciphertext("v_988") * Ciphertext("v_637")) * (Ciphertext("v_468") * Ciphertext("v_87"))) * ((Ciphertext("v_862") * Ciphertext("v_52")) * (Ciphertext("v_199") * Ciphertext("v_544")))) * (((Ciphertext("v_421") * Ciphertext("v_553")) * (Ciphertext("v_716") * Ciphertext("v_652"))) * ((Ciphertext("v_141") * Ciphertext("v_743")) * (Ciphertext("v_135") * Ciphertext("v_999")))))) * (((((Ciphertext("v_992") * Ciphertext("v_528")) * (Ciphertext("v_731") * Ciphertext("v_330"))) * ((Ciphertext("v_42") * Ciphertext("v_814")) * (Ciphertext("v_408") * Ciphertext("v_277")))) * (((Ciphertext("v_443") * Ciphertext("v_449")) * (Ciphertext("v_761") * Ciphertext("v_80"))) * ((Ciphertext("v_256") * Ciphertext("v_25")) * (Ciphertext("v_188") * Ciphertext("v_985"))))) * ((((Ciphertext("v_82") * Ciphertext("v_856")) * (Ciphertext("v_438") * Ciphertext("v_712"))) * ((Ciphertext("v_705") * Ciphertext("v_418")) * (Ciphertext("v_59") * Ciphertext("v_177")))) * (((Ciphertext("v_937") * Ciphertext("v_160")) * (Ciphertext("v_452") * Ciphertext("v_256"))) * ((Ciphertext("v_245") * Ciphertext("v_562")) * (Ciphertext("v_96") * Ciphertext("v_731")))))))) * (((((((Ciphertext("v_187") * Ciphertext("v_676")) * (Ciphertext("v_957") * Ciphertext("v_286"))) * ((Ciphertext("v_653") * Ciphertext("v_977")) * (Ciphertext("v_529") * Ciphertext("v_755")))) * (((Ciphertext("v_672") * Ciphertext("v_567")) * (Ciphertext("v_446") * Ciphertext("v_626"))) * ((Ciphertext("v_747") * Ciphertext("v_429")) * (Ciphertext("v_48") * Ciphertext("v_253"))))) * ((((Ciphertext("v_579") * Ciphertext("v_364")) * (Ciphertext("v_162") * Ciphertext("v_776"))) * ((Ciphertext("v_123") * Ciphertext("v_14")) * (Ciphertext("v_350") * Ciphertext("v_118")))) * (((Ciphertext("v_94") * Ciphertext("v_47")) * (Ciphertext("v_96") * Ciphertext("v_41"))) * ((Ciphertext("v_318") * Ciphertext("v_875")) * (Ciphertext("v_1003") * Ciphertext("v_720")))))) * (((((Ciphertext("v_179") * Ciphertext("v_485")) * (Ciphertext("v_17") * Ciphertext("v_100"))) * ((Ciphertext("v_781") * Ciphertext("v_503")) * (Ciphertext("v_826") * Ciphertext("v_484")))) * (((Ciphertext("v_354") * Ciphertext("v_177")) * (Ciphertext("v_475") * Ciphertext("v_444"))) * ((Ciphertext("v_665") * Ciphertext("v_408")) * (Ciphertext("v_49") * Ciphertext("v_630"))))) * ((((Ciphertext("v_971") * Ciphertext("v_650")) * (Ciphertext("v_907") * Ciphertext("v_46"))) * ((Ciphertext("v_267") * Ciphertext("v_383")) * (Ciphertext("v_15") * Ciphertext("v_640")))) * (((Ciphertext("v_955") * Ciphertext("v_515")) * (Ciphertext("v_513") * Ciphertext("v_877"))) * ((Ciphertext("v_552") * Ciphertext("v_674")) * (Ciphertext("v_700") * Ciphertext("v_166"))))))) * ((((((Ciphertext("v_630") * Ciphertext("v_72")) * (Ciphertext("v_966") * Ciphertext("v_657"))) * ((Ciphertext("v_966") * Ciphertext("v_27")) * (Ciphertext("v_888") * Ciphertext("v_599")))) * (((Ciphertext("v_651") * Ciphertext("v_962")) * (Ciphertext("v_820") * Ciphertext("v_589"))) * ((Ciphertext("v_693") * Ciphertext("v_397")) * (Ciphertext("v_587") * Ciphertext("v_658"))))) * ((((Ciphertext("v_131") * Ciphertext("v_469")) * (Ciphertext("v_830") * Ciphertext("v_558"))) * ((Ciphertext("v_840") * Ciphertext("v_964")) * (Ciphertext("v_640") * Ciphertext("v_395")))) * (((Ciphertext("v_840") * Ciphertext("v_984")) * (Ciphertext("v_667") * Ciphertext("v_927"))) * ((Ciphertext("v_211") * Ciphertext("v_200")) * (Ciphertext("v_280") * Ciphertext("v_71")))))) * (((((Ciphertext("v_219") * Ciphertext("v_103")) * (Ciphertext("v_386") * Ciphertext("v_18"))) * ((Ciphertext("v_39") * Ciphertext("v_96")) * (Ciphertext("v_633") * Ciphertext("v_699")))) * (((Ciphertext("v_344") * Ciphertext("v_72")) * (Ciphertext("v_671") * Ciphertext("v_326"))) * ((Ciphertext("v_394") * Ciphertext("v_865")) * (Ciphertext("v_697") * Ciphertext("v_23"))))) * ((((Ciphertext("v_348") * Ciphertext("v_834")) * (Ciphertext("v_266") * Ciphertext("v_288"))) * ((Ciphertext("v_507") * Ciphertext("v_121")) * (Ciphertext("v_709") * Ciphertext("v_713")))) * (((Ciphertext("v_356") * Ciphertext("v_644")) * (Ciphertext("v_959") * Ciphertext("v_898"))) * ((Ciphertext("v_40") * Ciphertext("v_752")) * (Ciphertext("v_427") * Ciphertext("v_822"))))))))) + ((((((((Ciphertext("v_990") * Ciphertext("v_288")) * (Ciphertext("v_561") * Ciphertext("v_181"))) * ((Ciphertext("v_433") * Ciphertext("v_768")) * (Ciphertext("v_582") * Ciphertext("v_826")))) * (((Ciphertext("v_589") * Ciphertext("v_1014")) * (Ciphertext("v_749") * Ciphertext("v_422"))) * ((Ciphertext("v_130") * Ciphertext("v_380")) * (Ciphertext("v_256") * Ciphertext("v_795"))))) * ((((Ciphertext("v_594") * Ciphertext("v_348")) * (Ciphertext("v_887") * Ciphertext("v_953"))) * ((Ciphertext("v_575") * Ciphertext("v_879")) * (Ciphertext("v_244") * Ciphertext("v_439")))) * (((Ciphertext("v_696") * Ciphertext("v_768")) * (Ciphertext("v_757") * Ciphertext("v_772"))) * ((Ciphertext("v_243") * Ciphertext("v_105")) * (Ciphertext("v_386") * Ciphertext("v_924")))))) * (((((Ciphertext("v_547") * Ciphertext("v_517")) * (Ciphertext("v_508") * Ciphertext("v_846"))) * ((Ciphertext("v_398") * Ciphertext("v_327")) * (Ciphertext("v_304") * Ciphertext("v_917")))) * (((Ciphertext("v_102") * Ciphertext("v_398")) * (Ciphertext("v_54") * Ciphertext("v_11"))) * ((Ciphertext("v_981") * Ciphertext("v_927")) * (Ciphertext("v_867") * Ciphertext("v_93"))))) * ((((Ciphertext("v_363") * Ciphertext("v_326")) * (Ciphertext("v_979") * Ciphertext("v_972"))) * ((Ciphertext("v_693") * Ciphertext("v_973")) * (Ciphertext("v_494") * Ciphertext("v_587")))) * (((Ciphertext("v_504") * Ciphertext("v_647")) * (Ciphertext("v_85") * Ciphertext("v_459"))) * ((Ciphertext("v_93") * Ciphertext("v_300")) * (Ciphertext("v_616") * Ciphertext("v_336"))))))) * ((((((Ciphertext("v_566") * Ciphertext("v_688")) * (Ciphertext("v_217") * Ciphertext("v_770"))) * ((Ciphertext("v_968") * Ciphertext("v_810")) * (Ciphertext("v_800") * Ciphertext("v_808")))) * (((Ciphertext("v_116") * Ciphertext("v_93")) * (Ciphertext("v_953") * Ciphertext("v_506"))) * ((Ciphertext("v_997") * Ciphertext("v_58")) * (Ciphertext("v_1019") * Ciphertext("v_562"))))) * ((((Ciphertext("v_57") * Ciphertext("v_452")) * (Ciphertext("v_754") * Ciphertext("v_909"))) * ((Ciphertext("v_559") * Ciphertext("v_359")) * (Ciphertext("v_722") * Ciphertext("v_119")))) * (((Ciphertext("v_991") * Ciphertext("v_146")) * (Ciphertext("v_704") * Ciphertext("v_779"))) * ((Ciphertext("v_857") * Ciphertext("v_633")) * (Ciphertext("v_887") * Ciphertext("v_103")))))) * (((((Ciphertext("v_858") * Ciphertext("v_86")) * (Ciphertext("v_436") * Ciphertext("v_717"))) * ((Ciphertext("v_227") * Ciphertext("v_74")) * (Ciphertext("v_406") * Ciphertext("v_841")))) * (((Ciphertext("v_913") * Ciphertext("v_723")) * (Ciphertext("v_150") * Ciphertext("v_83"))) * ((Ciphertext("v_525") * Ciphertext("v_86")) * (Ciphertext("v_238") * Ciphertext("v_1010"))))) * ((((Ciphertext("v_595") * Ciphertext("v_383")) * (Ciphertext("v_883") * Ciphertext("v_379"))) * ((Ciphertext("v_901") * Ciphertext("v_832")) * (Ciphertext("v_242") * Ciphertext("v_969")))) * (((Ciphertext("v_929") * Ciphertext("v_1009")) * (Ciphertext("v_606") * Ciphertext("v_771"))) * ((Ciphertext("v_358") * Ciphertext("v_385")) * (Ciphertext("v_725") * Ciphertext("v_85")))))))) * (((((((Ciphertext("v_493") * Ciphertext("v_795")) * (Ciphertext("v_387") * Ciphertext("v_571"))) * ((Ciphertext("v_722") * Ciphertext("v_434")) * (Ciphertext("v_254") * Ciphertext("v_928")))) * (((Ciphertext("v_338") * Ciphertext("v_113")) * (Ciphertext("v_444") * Ciphertext("v_504"))) * ((Ciphertext("v_259") * Ciphertext("v_11")) * (Ciphertext("v_414") * Ciphertext("v_479"))))) * ((((Ciphertext("v_585") * Ciphertext("v_57")) * (Ciphertext("v_758") * Ciphertext("v_121"))) * ((Ciphertext("v_785") * Ciphertext("v_120")) * (Ciphertext("v_842") * Ciphertext("v_664")))) * (((Ciphertext("v_762") * Ciphertext("v_763")) * (Ciphertext("v_697") * Ciphertext("v_597"))) * ((Ciphertext("v_243") * Ciphertext("v_985")) * (Ciphertext("v_409") * Ciphertext("v_383")))))) * (((((Ciphertext("v_489") * Ciphertext("v_292")) * (Ciphertext("v_84") * Ciphertext("v_889"))) * ((Ciphertext("v_541") * Ciphertext("v_228")) * (Ciphertext("v_446") * Ciphertext("v_986")))) * (((Ciphertext("v_821") * Ciphertext("v_12")) * (Ciphertext("v_1001") * Ciphertext("v_993"))) * ((Ciphertext("v_569") * Ciphertext("v_44")) * (Ciphertext("v_330") * Ciphertext("v_912"))))) * ((((Ciphertext("v_44") * Ciphertext("v_540")) * (Ciphertext("v_946") * Ciphertext("v_1016"))) * ((Ciphertext("v_941") * Ciphertext("v_75")) * (Ciphertext("v_259") * Ciphertext("v_567")))) * (((Ciphertext("v_712") * Ciphertext("v_992")) * (Ciphertext("v_77") * Ciphertext("v_535"))) * ((Ciphertext("v_170") * Ciphertext("v_644")) * (Ciphertext("v_222") * Ciphertext("v_712"))))))) * ((((((Ciphertext("v_301") * Ciphertext("v_465")) * (Ciphertext("v_1019") * Ciphertext("v_642"))) * ((Ciphertext("v_713") * Ciphertext("v_598")) * (Ciphertext("v_393") * Ciphertext("v_922")))) * (((Ciphertext("v_754") * Ciphertext("v_46")) * (Ciphertext("v_937") * Ciphertext("v_287"))) * ((Ciphertext("v_981") * Ciphertext("v_973")) * (Ciphertext("v_59") * Ciphertext("v_911"))))) * ((((Ciphertext("v_403") * Ciphertext("v_466")) * (Ciphertext("v_109") * Ciphertext("v_635"))) * ((Ciphertext("v_343") * Ciphertext("v_284")) * (Ciphertext("v_173") * Ciphertext("v_192")))) * (((Ciphertext("v_662") * Ciphertext("v_474")) * (Ciphertext("v_397") * Ciphertext("v_361"))) * ((Ciphertext("v_234") * Ciphertext("v_597")) * (Ciphertext("v_272") * Ciphertext("v_79")))))) * (((((Ciphertext("v_368") * Ciphertext("v_997")) * (Ciphertext("v_266") * Ciphertext("v_1009"))) * ((Ciphertext("v_464") * Ciphertext("v_714")) * (Ciphertext("v_33") * Ciphertext("v_548")))) * (((Ciphertext("v_286") * Ciphertext("v_316")) * (Ciphertext("v_421") * Ciphertext("v_833"))) * ((Ciphertext("v_617") * Ciphertext("v_701")) * (Ciphertext("v_277") * Ciphertext("v_166"))))) * ((((Ciphertext("v_136") * Ciphertext("v_932")) * (Ciphertext("v_78") * Ciphertext("v_842"))) * ((Ciphertext("v_963") * Ciphertext("v_972")) * (Ciphertext("v_426") * Ciphertext("v_743")))) * (((Ciphertext("v_595") * Ciphertext("v_628")) * (Ciphertext("v_941") * Ciphertext("v_428"))) * ((Ciphertext("v_84") * Ciphertext("v_955")) * (Ciphertext("v_897") * Ciphertext("v_886"))))))))))));

    output.set_output("output");
}

/********************************************/
void print_bool_arg(bool arg, const string &name, ostream &os)
{
  os << (arg ? name : "no_" + name);
}
int main(int argc, char **argv)
{
  bool vectorized = true;
  if (argc > 1)
    vectorized = stoi(argv[1]);

  int window = 0;
  if (argc > 2) 
    window = stoi(argv[2]);

  bool call_quantifier = true;
  if (argc > 3)
    call_quantifier = stoi(argv[3]);

  bool cse = true;
  if (argc > 4)
    cse = stoi(argv[4]);
  
  int slot_count = 1 ;
  if (argc > 5)
    slot_count = stoi(argv[5]);

  bool const_folding = true;
  if (argc > 5)
    const_folding = stoi(argv[5]);

  if (cse)
  {
    Compiler::enable_cse();
    Compiler::enable_order_operands();
  }
  else
  {
    Compiler::disable_cse();
    Compiler::disable_order_operands();
  }

  if (const_folding)
    Compiler::enable_const_folding();
  else
    Compiler::disable_const_folding(); 

  chrono::high_resolution_clock::time_point t;
  chrono::duration<double, milli> elapsed;
  string func_name = "fhe";
  /**************/t = chrono::high_resolution_clock::now();
  if (vectorized)
  { 
      const auto &func = Compiler::create_func(func_name, 1, 20, true, true);
      fhe(slot_count);
      string gen_name = "_gen_he_" + func_name;
      string gen_path = "he/" + gen_name;
      ofstream header_os(gen_path + ".hpp");
      if (!header_os)
        throw logic_error("failed to create header file");
      ofstream source_os(gen_path + ".cpp");
      if (!source_os)
        throw logic_error("failed to create source file");
      cout << " window is " << window << endl;
      Compiler::gen_vectorized_code(func, window);
      auto ruleset = Compiler::Ruleset::ops_cost;
      auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
      Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);  
      Compiler::gen_he_code(func, header_os, gen_name + ".hpp", source_os);    
      /************/elapsed = chrono::high_resolution_clock::now() - t;   
      cout << elapsed.count() << " ms\n";
      if (call_quantifier)
      {
        util::Quantifier quantifier{func};
        quantifier.run_all_analysis();
        quantifier.print_info(cout);
      }
  }
  else
  {
      const auto &func = Compiler::create_func(func_name, slot_count, 20, true, true);
      fhe_vectorized(slot_count);
      string gen_name = "_gen_he_" + func_name;
      string gen_path = "he/" + gen_name;
      ofstream header_os(gen_path + ".hpp");
      if (!header_os)
        throw logic_error("failed to create header file");
      ofstream source_os(gen_path + ".cpp");
      if (!source_os)
        throw logic_error("failed to create source file");
      auto ruleset = Compiler::Ruleset::ops_cost;
      auto rewrite_heuristic = trs::RewriteHeuristic::bottom_up;
      Compiler::compile(func, ruleset, rewrite_heuristic, header_os, gen_name + ".hpp", source_os);
      /************/elapsed = chrono::high_resolution_clock::now() - t;
      cout << elapsed.count() << " ms\n";
      if (call_quantifier)
      {
        util::Quantifier quantifier{func};
        quantifier.run_all_analysis();
        quantifier.print_info(cout);
      }
  }
  return 0;
}