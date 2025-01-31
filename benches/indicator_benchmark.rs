extern crate criterion;
extern crate my_project;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use my_project::utilities::data_loader::read_candles_from_csv;

use my_project::indicators::{
    acosc::{acosc, AcoscInput},
    ad::{ad, AdInput},
    adosc::{adosc, AdoscInput},
    adx::{adx, AdxInput},
    adxr::{adxr, AdxrInput},
    alligator::{alligator, AlligatorInput},
    alma::{alma, AlmaInput},
    ao::{ao, AoInput},
    apo::{apo, ApoInput},
    aroon::{aroon, AroonInput},
    aroonosc::{aroon_osc, AroonOscInput},
    atr::{atr, AtrInput},
    avgprice::{avgprice, AvgPriceInput},
    bandpass::{bandpass, BandPassInput},
    bollinger_bands::{bollinger_bands, BollingerBandsInput},
    bollinger_bands_width::{bollinger_bands_width, BollingerBandsWidthInput},
    bop::{bop, BopInput},
    cci::{cci, CciInput},
    cfo::{cfo, CfoInput},
    cg::{cg, CgInput},
    chande::{chande, ChandeInput},
    chop::{chop, ChopInput},
    cksp::{cksp, CkspInput},
    cmo::{cmo, CmoInput},
    coppock::{coppock, CoppockInput},
    correl_hl::{correl_hl, CorrelHlInput},
    correlation_cycle::{correlation_cycle, CorrelationCycleInput},
    cvi::{cvi, CviInput},
    cwma::{cwma, CwmaInput},
    damiani_volatmeter::{damiani_volatmeter, DamianiVolatmeterInput},
    dec_osc::{dec_osc, DecOscInput},
    decycler::{decycler, DecyclerInput},
    dema::{dema, DemaInput},
    devstop::{devstop, DevStopInput},
    di::{di, DiInput},
    dm::{dm, DmInput},
    donchian::{donchian, DonchianInput},
    dpo::{dpo, DpoInput},
    dti::{dti, DtiInput},
    dx::{dx, DxInput},
    edcf::{edcf, EdcfInput},
    efi::{efi, EfiInput},
    ehlers_itrend::{ehlers_itrend, EhlersITrendInput},
    ema::{ema, EmaInput},
    emd::{emd, EmdInput},
    emv::{emv, EmvInput},
    epma::{epma, EpmaInput},
    er::{er, ErInput},
    eri::{eri, EriInput},
    fisher::{fisher, FisherInput},
    fosc::{fosc, FoscInput},
    frama::{frama, FramaInput},
    fwma::{fwma, FwmaInput},
    gatorosc::{gatorosc, GatorOscInput},
    gaussian::{gaussian, GaussianInput},
    heikin_ashi_candles::{heikin_ashi_candles, HeikinAshiInput},
    highpass::{highpass, HighPassInput},
    highpass_2_pole::{highpass_2_pole, HighPass2Input},
    hma::{hma, HmaInput},
    ht_dcperiod::{ht_dcperiod, HtDcPeriodInput},
    hwma::{hwma, HwmaInput},
    ift_rsi::{ift_rsi, IftRsiInput},
    jma::{jma, JmaInput},
    jsa::{jsa, JsaInput},
    kama::{kama, KamaInput},
    kaufmanstop::{kaufmanstop, KaufmanstopInput},
    kdj::{kdj, KdjInput},
    keltner::{keltner, KeltnerInput},
    kst::{kst, KstInput},
    kurtosis::{kurtosis, KurtosisInput},
    kvo::{kvo, KvoInput},
    linearreg_angle::{linearreg_angle, Linearreg_angleInput},
    linearreg_intercept::{linearreg_intercept, LinearRegInterceptInput},
    linearreg_slope::{linearreg_slope, LinearRegSlopeInput},
    linreg::{linreg, LinRegInput},
    lrsi::{lrsi, LrsiInput},
    maaq::{maaq, MaaqInput},
    mab::{mab, MabInput},
    macd::{macd, MacdInput},
    mama::{mama, MamaInput},
    marketefi::{marketfi, MarketefiInput},
    mass::{mass, MassInput},
    mean_ad::{mean_ad, MeanAdInput},
    medium_ad::{medium_ad, MediumAdInput},
    medprice::{medprice, MedpriceInput},
    mfi::{mfi, MfiInput},
    midpoint::{midpoint, MidpointInput},
    midprice::{midprice, MidpriceInput},
    minmax::{minmax, MinmaxInput},
    mom::{mom, MomInput},
    msw::{msw, MswInput},
    mwdx::{mwdx, MwdxInput},
    natr::{natr, NatrInput},
    nma::{nma, NmaInput},
    pivot::{pivot, PivotInput},
    pma::{pma, PmaInput},
    ppo::{ppo, PpoInput},
    pvi::{pvi, PviInput},
    pwma::{pwma, PwmaInput},
    qstick::{qstick, QstickInput},
    reflex::{reflex, ReflexInput},
    roc::{roc, RocInput},
    rocp::{rocp, RocpInput},
    rocr::{rocr, RocrInput},
    rsi::{rsi, RsiInput},
    rvi::{rvi, RviInput},
    safezonestop::{safezonestop, SafeZoneStopInput},
    sar::{sar, SarInput},
    sinwma::{sinwma, SinWmaInput},
    sma::{sma, SmaInput},
    smma::{smma, SmmaInput},
    squeeze_momentum::{squeeze_momentum, SqueezeMomentumInput},
    sqwma::{sqwma, SqwmaInput},
    srsi::{srsi, SrsiInput},
    srwma::{srwma, SrwmaInput},
    stc::{stc, StcInput},
    stddev::{stddev, StdDevInput},
    stochf::{stochf, StochfInput},
    supersmoother::{supersmoother, SuperSmootherInput},
    supersmoother_3_pole::{supersmoother_3_pole, SuperSmoother3PoleInput},
    swma::{swma, SwmaInput},
    tema::{tema, TemaInput},
    tilson::{tilson, TilsonInput},
    trendflex::{trendflex, TrendFlexInput},
    trima::{trima, TrimaInput},
    trix::{trix, TrixInput},
    tsi::{tsi, TsiInput},
    ttm_trend::{ttm_trend, TtmTrendInput},
    ui::{ui, UiInput},
    ultosc::{ultosc, UltOscInput},
    var::{var, VarInput},
    vi::{vi, ViInput},
    vidya::{vidya, VidyaInput},
    vlma::{vlma, VlmaInput},
    vosc::{vosc, VoscInput},
    voss::{voss, VossInput},
    vpci::{vpci, VpciInput},
    vpt::{vpt, VptInput},
    vpwma::{vpwma, VpwmaInput},
    vwap::{vwap, VwapInput},
    vwma::{vwma, VwmaInput},
    vwmacd::{vwmacd, VwmacdInput},
    wad::{wad, WadInput},
    wavetrend::{wavetrend, WavetrendInput},
    wclprice::{wclprice, WclpriceInput},
    wilders::{wilders, WildersInput},
    willr::{willr, WillrInput},
    wma::{wma, WmaInput},
    zlema::{zlema, ZlemaInput},
    zscore::{zscore, ZscoreInput},
};
use std::time::Duration;

fn benchmark_indicators(c: &mut Criterion) {
    let candles =
        read_candles_from_csv("src/data/bitfinex btc-usd 100,000 candles ends 09-01-24.csv")
            .expect("Failed to load candles");

    let mut group = c.benchmark_group("Indicator Benchmarks");
    group.measurement_time(Duration::new(8, 0));
    group.warm_up_time(Duration::new(4, 0));

    // ZSCORE
    group.bench_function(BenchmarkId::new("ZSCORE", 0), |b| {
        let input = ZscoreInput::with_default_candles(&candles);
        b.iter(|| zscore(black_box(&input)).expect("Failed to calculate ZSCORE"))
    });

    // WILLR
    group.bench_function(BenchmarkId::new("WILLR", 0), |b| {
        let input = WillrInput::with_default_candles(&candles);
        b.iter(|| willr(black_box(&input)).expect("Failed to calculate WILLR"))
    });

    // WCLPRICE
    group.bench_function(BenchmarkId::new("WCLPRICE", 0), |b| {
        let input = WclpriceInput::with_default_candles(&candles);
        b.iter(|| wclprice(black_box(&input)).expect("Failed to calculate WCLPRICE"))
    });

    // WAVE TREND
    group.bench_function(BenchmarkId::new("WAVETREND", 0), |b| {
        let input = WavetrendInput::with_default_candles(&candles);
        b.iter(|| wavetrend(black_box(&input)).expect("Failed to calculate Wave Trend"))
    });

    //WAD
    group.bench_function(BenchmarkId::new("WAD", 0), |b| {
        let input = WadInput::with_default_candles(&candles);
        b.iter(|| wad(black_box(&input)).expect("Failed to calculate WAD"))
    });

    // VWMACD
    group.bench_function(BenchmarkId::new("VWMACD", 0), |b| {
        let input = VwmacdInput::with_default_candles(&candles);
        b.iter(|| vwmacd(black_box(&input)).expect("Failed to calculate VWMACD"))
    });

    // VPT
    group.bench_function(BenchmarkId::new("VPT", 0), |b| {
        let input = VptInput::with_default_candles(&candles);
        b.iter(|| vpt(black_box(&input)).expect("Failed to calculate VPT"))
    });

    //VPCI
    group.bench_function(BenchmarkId::new("VPCI", 0), |b| {
        let input = VpciInput::with_default_candles(&candles);
        b.iter(|| vpci(black_box(&input)).expect("Failed to calculate VPCI"))
    });

    // VOSS
    group.bench_function(BenchmarkId::new("VOSS", 0), |b| {
        let input = VossInput::with_default_candles(&candles);
        b.iter(|| voss(black_box(&input)).expect("Failed to calculate VOSS"))
    });

    // VOSC
    group.bench_function(BenchmarkId::new("VOSC", 0), |b| {
        let input = VoscInput::with_default_candles(&candles);
        b.iter(|| vosc(black_box(&input)).expect("Failed to calculate VOSC"))
    });

    // VIDYA
    group.bench_function(BenchmarkId::new("VIDYA", 0), |b| {
        let input = VidyaInput::with_default_candles(&candles);
        b.iter(|| vidya(black_box(&input)).expect("Failed to calculate VIDYA"))
    });

    // ULTOSC
    group.bench_function(BenchmarkId::new("ULTOSC", 0), |b| {
        let input = UltOscInput::with_default_candles(&candles);
        b.iter(|| ultosc(black_box(&input)).expect("Failed to calculate ULTOSC"))
    });

    // VLMA
    group.bench_function(BenchmarkId::new("VLMA", 0), |b| {
        let input = VlmaInput::with_default_candles(&candles);
        b.iter(|| vlma(black_box(&input)).expect("Failed to calculate VLMA"))
    });

    // VI
    group.bench_function(BenchmarkId::new("VI", 0), |b| {
        let input = ViInput::with_default_candles(&candles);
        b.iter(|| vi(black_box(&input)).expect("Failed to calculate VI"))
    });

    // VAR
    group.bench_function(BenchmarkId::new("VAR", 0), |b| {
        let input = VarInput::with_default_candles(&candles);
        b.iter(|| var(black_box(&input)).expect("Failed to calculate VAR"))
    });

    // TRIX
    group.bench_function(BenchmarkId::new("TRIX", 0), |b| {
        let input = TrixInput::with_default_candles(&candles);
        b.iter(|| trix(black_box(&input)).expect("Failed to calculate TRIX"))
    });

    // TSI
    group.bench_function(BenchmarkId::new("TSI", 0), |b| {
        let input = TsiInput::with_default_candles(&candles);
        b.iter(|| tsi(black_box(&input)).expect("Failed to calculate TSI"))
    });

    // TTM Trend
    group.bench_function(BenchmarkId::new("TTM_TREND", 0), |b| {
        let input = TtmTrendInput::with_default_candles(&candles);
        b.iter(|| ttm_trend(black_box(&input)).expect("Failed to calculate TTM Trend"))
    });

    //UI
    group.bench_function(BenchmarkId::new("UI", 0), |b| {
        let input = UiInput::with_default_candles(&candles);
        b.iter(|| ui(black_box(&input)).expect("Failed to calculate UI"))
    });

    // Stochf
    group.bench_function(BenchmarkId::new("STOCHF", 0), |b| {
        let input = StochfInput::with_default_candles(&candles);
        b.iter(|| stochf(black_box(&input)).expect("Failed to calculate STOCHF"))
    });

    // STC
    group.bench_function(BenchmarkId::new("STC", 0), |b| {
        let input = StcInput::with_default_candles(&candles);
        b.iter(|| stc(black_box(&input)).expect("Failed to calculate STC"))
    });

    // STDDEV
    group.bench_function(BenchmarkId::new("STDDEV", 0), |b| {
        let input = StdDevInput::with_default_candles(&candles);
        b.iter(|| stddev(black_box(&input)).expect("Failed to calculate STDDEV"))
    });

    // SRSI
    group.bench_function(BenchmarkId::new("SRSI", 0), |b| {
        let input = SrsiInput::with_default_candles(&candles);
        b.iter(|| srsi(black_box(&input)).expect("Failed to calculate SRSI"))
    });

    // Squeeze Momentum
    group.bench_function(BenchmarkId::new("SQUEEZE_MOMENTUM", 0), |b| {
        let input = SqueezeMomentumInput::with_default_candles(&candles);
        b.iter(|| {
            squeeze_momentum(black_box(&input)).expect("Failed to calculate Squeeze Momentum")
        })
    });

    // RVI
    group.bench_function(BenchmarkId::new("RVI", 0), |b| {
        let input = RviInput::with_default_candles(&candles);
        b.iter(|| rvi(black_box(&input)).expect("Failed to calculate RVI"))
    });

    // Sar
    group.bench_function(BenchmarkId::new("SAR", 0), |b| {
        let input = SarInput::with_default_candles(&candles).expect("Failed to create SarInput");
        b.iter(|| sar(black_box(&input)).expect("Failed to calculate SAR"))
    });

    // SafeZoneStop
    group.bench_function(BenchmarkId::new("SAFEZONESTOP", 0), |b| {
        let input = SafeZoneStopInput::with_default_candles_long(&candles);
        b.iter(|| safezonestop(black_box(&input)).expect("Failed to calculate SafeZoneStop"))
    });

    // PVI
    group.bench_function(BenchmarkId::new("PVI", 0), |b| {
        let input = PviInput::with_default_candles(&candles);
        b.iter(|| pvi(black_box(&input)).expect("Failed to calculate PVI"))
    });

    // PPO
    group.bench_function(BenchmarkId::new("PPO", 0), |b| {
        let input = PpoInput::with_default_candles(&candles);
        b.iter(|| ppo(black_box(&input)).expect("Failed to calculate PPO"))
    });

    // Qstick
    group.bench_function(BenchmarkId::new("QSTICK", 0), |b| {
        let input = QstickInput::with_default_candles(&candles);
        b.iter(|| qstick(black_box(&input)).expect("Failed to calculate QSTICK"))
    });

    // PMA
    group.bench_function(BenchmarkId::new("PMA", 0), |b| {
        let input = PmaInput::with_default_candles(&candles);
        b.iter(|| pma(black_box(&input)).expect("Failed to calculate PMA"))
    });

    // NATR
    group.bench_function(BenchmarkId::new("NATR", 0), |b| {
        let input = NatrInput::with_default_candles(&candles);
        b.iter(|| natr(black_box(&input)).expect("Failed to calculate NATR"))
    });

    // PIVOT
    group.bench_function(BenchmarkId::new("PIVOT", 0), |b| {
        let input = PivotInput::with_default_candles(&candles);
        b.iter(|| pivot(black_box(&input)).expect("Failed to calculate PIVOT"))
    });

    // MIDPRICE
    group.bench_function(BenchmarkId::new("MIDPRICE", 0), |b| {
        let input = MidpriceInput::with_default_candles(&candles);
        b.iter(|| midprice(black_box(&input)).expect("Failed to calculate MIDPRICE"))
    });

    // MINMAX
    group.bench_function(BenchmarkId::new("MINMAX", 0), |b| {
        let input = MinmaxInput::with_default_candles(&candles);
        b.iter(|| minmax(black_box(&input)).expect("Failed to calculate MINMAX"))
    });

    // MOM
    group.bench_function(BenchmarkId::new("MOM", 0), |b| {
        let input = MomInput::with_default_candles(&candles);
        b.iter(|| mom(black_box(&input)).expect("Failed to calculate MOM"))
    });

    // MSW
    group.bench_function(BenchmarkId::new("MSW", 0), |b| {
        let input = MswInput::with_default_candles(&candles);
        b.iter(|| msw(black_box(&input)).expect("Failed to calculate MSW"))
    });

    // MIDPOINT
    group.bench_function(BenchmarkId::new("MIDPOINT", 0), |b| {
        let input = MidpointInput::with_default_candles(&candles);
        b.iter(|| midpoint(black_box(&input)).expect("Failed to calculate MIDPOINT"))
    });

    // MFI
    group.bench_function(BenchmarkId::new("MFI", 0), |b| {
        let input = MfiInput::with_default_candles(&candles);
        b.iter(|| mfi(black_box(&input)).expect("Failed to calculate MFI"))
    });

    // Medprice
    group.bench_function(BenchmarkId::new("MEDPRICE", 0), |b| {
        let input = MedpriceInput::with_default_candles(&candles);
        b.iter(|| medprice(black_box(&input)).expect("Failed to calculate MEDPRICE"))
    });

    // Medium AD
    group.bench_function(BenchmarkId::new("MEDIUM_AD", 0), |b| {
        let input = MediumAdInput::with_default_candles(&candles);
        b.iter(|| medium_ad(black_box(&input)).expect("Failed to calculate MEDIUM_AD"))
    });

    // Kurtosis
    group.bench_function(BenchmarkId::new("KURTOSIS", 0), |b| {
        let input = KurtosisInput::with_default_candles(&candles);
        b.iter(|| kurtosis(black_box(&input)).expect("Failed to calculate KURTOSIS"))
    });

    // Market EFI
    group.bench_function(BenchmarkId::new("MARKETEFI", 0), |b| {
        let input = MarketefiInput::with_default_candles(&candles);
        b.iter(|| marketfi(black_box(&input)).expect("Failed to calculate MARKETEFI"))
    });

    // MEAN AD
    group.bench_function(BenchmarkId::new("MEAN_AD", 0), |b| {
        let input = MeanAdInput::with_default_candles(&candles);
        b.iter(|| mean_ad(black_box(&input)).expect("Failed to calculate MEAN_AD"))
    });

    // MASS
    group.bench_function(BenchmarkId::new("MASS", 0), |b| {
        let input = MassInput::with_default_candles(&candles);
        b.iter(|| mass(black_box(&input)).expect("Failed to calculate MASS"))
    });

    // MAB
    group.bench_function(BenchmarkId::new("MAB", 0), |b| {
        let input = MabInput::with_default_candles(&candles);
        b.iter(|| mab(black_box(&input)).expect("Failed to calculate MAB"))
    });

    // MACD
    group.bench_function(BenchmarkId::new("MACD", 0), |b| {
        let input = MacdInput::with_default_candles(&candles);
        b.iter(|| macd(black_box(&input)).expect("Failed to calculate MACD"))
    });

    // Linear Regression Slope
    group.bench_function(BenchmarkId::new("LINEARREG_SLOPE", 0), |b| {
        let input = LinearRegSlopeInput::with_default_candles(&candles);
        b.iter(|| linearreg_slope(black_box(&input)).expect("Failed to calculate LINEARREG_SLOPE"))
    });

    // Linear regression intercept
    group.bench_function(BenchmarkId::new("LINEARREG_INTERCEPT", 0), |b| {
        let input = LinearRegInterceptInput::with_default_candles(&candles);
        b.iter(|| {
            linearreg_intercept(black_box(&input)).expect("Failed to calculate LINEARREG_INTERCEPT")
        })
    });

    // LRSI
    group.bench_function(BenchmarkId::new("LRSI", 0), |b| {
        let input = LrsiInput::with_default_candles(&candles);
        b.iter(|| lrsi(black_box(&input)).expect("Failed to calculate LRSI"))
    });

    // KVO
    group.bench_function(BenchmarkId::new("KVO", 0), |b| {
        let input = KvoInput::with_default_candles(&candles);
        b.iter(|| kvo(black_box(&input)).expect("Failed to calculate KVO"))
    });

    // Linear Regression Angle
    group.bench_function(BenchmarkId::new("LINEARREG_ANGLE", 0), |b| {
        let input = Linearreg_angleInput::with_default_candles(&candles);
        b.iter(|| linearreg_angle(black_box(&input)).expect("Failed to calculate LINEARREG_ANGLE"))
    });

    // keltern Channel
    group.bench_function(BenchmarkId::new("KELTNER", 0), |b| {
        let input = KeltnerInput::with_default_candles(&candles);
        b.iter(|| keltner(black_box(&input)).expect("Failed to calculate KELTNER"))
    });

    // kaufman Stop
    group.bench_function(BenchmarkId::new("KaufmanStop", 0), |b| {
        let input = KaufmanstopInput::with_default_candles(&candles);
        b.iter(|| kaufmanstop(black_box(&input)).expect("Failed to calculate Kaufman Stop"))
    });

    // kst
    group.bench_function(BenchmarkId::new("KST", 0), |b| {
        let input = KstInput::with_default_candles(&candles);
        b.iter(|| kst(black_box(&input)).expect("Failed to calculate KST"))
    });

    // KDJ
    group.bench_function(BenchmarkId::new("KDJ", 0), |b| {
        let input = KdjInput::with_default_candles(&candles);
        b.iter(|| kdj(black_box(&input)).expect("Failed to calculate KDJ"))
    });

    // IFT RSI
    group.bench_function(BenchmarkId::new("IFT_RSI", 0), |b| {
        let input = IftRsiInput::with_default_candles(&candles);
        b.iter(|| ift_rsi(black_box(&input)).expect("Failed to calculate IFT_RSI"))
    });

    // HT_DCPeriod
    group.bench_function(BenchmarkId::new("HT_DCPeriod", 0), |b| {
        let input = HtDcPeriodInput::with_default_candles(&candles);
        b.iter(|| ht_dcperiod(black_box(&input)).expect("Failed to calculate HT_DCPeriod"))
    });

    // Heikin Ashi Candles
    group.bench_function(BenchmarkId::new("HEIKIN_ASHI", 0), |b| {
        let input = HeikinAshiInput::with_default_candles(&candles);
        b.iter(|| heikin_ashi_candles(black_box(&input)).expect("Failed to calculate Heikin Ashi"))
    });

    // FRAMA
    group.bench_function(BenchmarkId::new("FRAMA", 0), |b| {
        let input = FramaInput::with_default_candles(&candles);
        b.iter(|| frama(black_box(&input)).expect("Failed to calculate FRAMA"))
    });

    // FOSC
    group.bench_function(BenchmarkId::new("FOSC", 0), |b| {
        let input = FoscInput::with_default_candles(&candles);
        b.iter(|| fosc(black_box(&input)).expect("Failed to calculate FOSC"))
    });

    // Gator Oscillator
    group.bench_function(BenchmarkId::new("GATOROSC", 0), |b| {
        let input = GatorOscInput::with_default_candles(&candles);
        b.iter(|| gatorosc(black_box(&input)).expect("Failed to calculate GATOROSC"))
    });

    // FISHER
    group.bench_function(BenchmarkId::new("FISHER", 0), |b| {
        let input = FisherInput::with_default_candles(&candles);
        b.iter(|| fisher(black_box(&input)).expect("Failed to calculate Fisher Transform"))
    });

    // ERI
    group.bench_function(BenchmarkId::new("ERI", 0), |b| {
        let input = EriInput::with_default_candles(&candles);
        b.iter(|| eri(black_box(&input)).expect("Failed to calculate ERI"))
    });

    // ER
    group.bench_function(BenchmarkId::new("ER", 0), |b| {
        let input = ErInput::with_default_candles(&candles);
        b.iter(|| er(black_box(&input)).expect("Failed to calculate ER"))
    });

    // EMV
    group.bench_function(BenchmarkId::new("EMV", 0), |b| {
        let input = EmvInput::with_default_candles(&candles);
        b.iter(|| emv(black_box(&input)).expect("Failed to calculate EMV"))
    });

    // EMD
    group.bench_function(BenchmarkId::new("EMD", 0), |b| {
        let input = EmdInput::with_default_candles(&candles);
        b.iter(|| emd(black_box(&input)).expect("Failed to calculate EMD"))
    });

    // EFI
    group.bench_function(BenchmarkId::new("EFI", 0), |b| {
        let input = EfiInput::with_default_candles(&candles);
        b.iter(|| efi(black_box(&input)).expect("Failed to calculate EFI"))
    });

    // DX
    group.bench_function(BenchmarkId::new("DX", 0), |b| {
        let input = DxInput::with_default_candles(&candles);
        b.iter(|| dx(black_box(&input)).expect("Failed to calculate DX"))
    });

    // DTI
    group.bench_function(BenchmarkId::new("DTI", 0), |b| {
        let input = DtiInput::with_default_candles(&candles);
        b.iter(|| dti(black_box(&input)).expect("Failed to calculate DTI"))
    });

    // Detrended Price Oscillator
    group.bench_function(BenchmarkId::new("DPO", 0), |b| {
        let input = DpoInput::with_default_candles(&candles);
        b.iter(|| dpo(black_box(&input)).expect("Failed to calculate DPO"))
    });

    // Donchian Channel
    group.bench_function(BenchmarkId::new("DONCHIAN", 0), |b| {
        let input = DonchianInput::with_default_candles(&candles);
        b.iter(|| donchian(black_box(&input)).expect("Failed to calculate DONCHIAN"))
    });

    // Directional Index
    group.bench_function(BenchmarkId::new("DI", 0), |b| {
        let input = DiInput::with_default_candles(&candles);
        b.iter(|| di(black_box(&input)).expect("Failed to calculate DI"))
    });

    // Directional Movement
    group.bench_function(BenchmarkId::new("DM", 0), |b| {
        let input = DmInput::with_default_candles(&candles);
        b.iter(|| dm(black_box(&input)).expect("Failed to calculate DM"))
    });

    // DEVIATION STOP
    group.bench_function(BenchmarkId::new("DEVIATION STOP", 0), |b| {
        let input = DevStopInput::with_default_candles(&candles);
        b.iter(|| devstop(black_box(&input)).expect("Failed to calculate DEVIATION STOP"))
    });

    // DECYCLER
    group.bench_function(BenchmarkId::new("DECYCLER", 0), |b| {
        let input = DecyclerInput::with_default_candles(&candles);
        b.iter(|| decycler(black_box(&input)).expect("Failed to calculate DECYCLER"))
    });
    // DECYCLER OSCILLATOR
    group.bench_function(BenchmarkId::new("DECYCLER OSCILLATOR", 0), |b| {
        let input = DecOscInput::with_default_candles(&candles);
        b.iter(|| dec_osc(black_box(&input)).expect("Failed to calculate DECYCLER"))
    });

    // CORRELATION_CYCLE
    group.bench_function(BenchmarkId::new("CORRELATION_CYCLE", 0), |b| {
        let input = CorrelationCycleInput::with_default_candles(&candles);
        b.iter(|| {
            correlation_cycle(black_box(&input)).expect("Failed to calculate CORRELATION_CYCLE")
        })
    });

    // CORREL_HL
    group.bench_function(BenchmarkId::new("CORREL_HL", 0), |b| {
        let input = CorrelHlInput::with_default_candles(&candles);
        b.iter(|| correl_hl(black_box(&input)).expect("Failed to calculate CORREL_HL"))
    });

    // DAMIANI VOLATMETER
    group.bench_function(BenchmarkId::new("DAMIANI_VOLATMETER", 0), |b| {
        let input = DamianiVolatmeterInput::with_default_candles(&candles);
        b.iter(|| {
            damiani_volatmeter(black_box(&input)).expect("Failed to calculate DAMIANI_VOLATMETER")
        })
    });

    //CVI
    group.bench_function(BenchmarkId::new("CVI", 0), |b| {
        let input = CviInput::with_default_candles(&candles);
        b.iter(|| cvi(black_box(&input)).expect("Failed to calculate CVI"))
    });

    // CHANDE
    group.bench_function(BenchmarkId::new("CHANDE", 0), |b| {
        let input = ChandeInput::with_default_candles(&candles);
        b.iter(|| chande(black_box(&input)).expect("Failed to calculate CHANDE"))
    });

    // CHOP
    group.bench_function(BenchmarkId::new("CHOP", 0), |b| {
        let input = ChopInput::with_default_candles(&candles);
        b.iter(|| chop(black_box(&input)).expect("Failed to calculate CHOP"))
    });

    // Chande Kroll Stop
    group.bench_function(BenchmarkId::new("CKSP", 0), |b| {
        let input = CkspInput::with_default_candles(&candles);
        b.iter(|| cksp(black_box(&input)).expect("Failed to calculate CKSP"))
    });

    // Chande Momentum Oscillator
    group.bench_function(BenchmarkId::new("CMO", 0), |b| {
        let input = CmoInput::with_default_candles(&candles);
        b.iter(|| cmo(black_box(&input)).expect("Failed to calculate CMO"))
    });

    // Center of Gravity
    group.bench_function(BenchmarkId::new("CG", 0), |b| {
        let input = CgInput::with_default_candles(&candles);
        b.iter(|| cg(black_box(&input)).expect("Failed to calculate CG"))
    });

    // Rate of Change Ratio
    group.bench_function(BenchmarkId::new("ROCR", 0), |b| {
        let input = RocrInput::with_default_candles(&candles);
        b.iter(|| rocr(black_box(&input)).expect("Failed to calculate ROCR"))
    });
    // Chnade Forecast Oscillator
    group.bench_function(BenchmarkId::new("CFO", 0), |b| {
        let input = CfoInput::with_default_candles(&candles);
        b.iter(|| cfo(black_box(&input)).expect("Failed to calculate CFO"))
    });

    // Coppock Curve
    group.bench_function(BenchmarkId::new("COPPOCK", 0), |b| {
        let input = CoppockInput::with_default_candles(&candles);
        b.iter(|| coppock(black_box(&input)).expect("Failed to calculate COPPOCK"))
    });

    // Bollinger Bands Width
    group.bench_function(BenchmarkId::new("BOLLINGER_BANDS_WIDTH", 0), |b| {
        let input = BollingerBandsWidthInput::with_default_candles(&candles);
        b.iter(|| {
            bollinger_bands_width(black_box(&input))
                .expect("Failed to calculate BOLLINGER_BANDS_WIDTH")
        })
    });

    // ROCP
    group.bench_function(BenchmarkId::new("ROCP", 0), |b| {
        let input = RocpInput::with_default_candles(&candles);
        b.iter(|| rocp(black_box(&input)).expect("Failed to calculate ROCP"))
    });

    // BOP
    group.bench_function(BenchmarkId::new("BOP", 0), |b| {
        let input = BopInput::with_default_candles(&candles);
        b.iter(|| bop(black_box(&input)).expect("Failed to calculate BOP"))
    });

    // CCI
    group.bench_function(BenchmarkId::new("CCI", 0), |b| {
        let input = CciInput::with_default_candles(&candles);
        b.iter(|| cci(black_box(&input)).expect("Failed to calculate CCI"))
    });

    // Bollinger Bands
    group.bench_function(BenchmarkId::new("BOLLINGER_BANDS", 0), |b| {
        let input = BollingerBandsInput::with_default_candles(&candles);
        b.iter(|| bollinger_bands(black_box(&input)).expect("Failed to calculate BOLLINGER_BANDS"))
    });

    // ROC
    group.bench_function(BenchmarkId::new("ROC", 0), |b| {
        let input = RocInput::with_default_candles(&candles);
        b.iter(|| roc(black_box(&input)).expect("Failed to calculate ROC"))
    });

    // EPMA
    group.bench_function(BenchmarkId::new("EPMA", 0), |b| {
        let input = EpmaInput::with_default_candles(&candles);
        b.iter(|| epma(black_box(&input)).expect("Failed to calculate EPMA"))
    });

    // JSA
    group.bench_function(BenchmarkId::new("JSA", 0), |b| {
        let input = JsaInput::with_default_candles(&candles);
        b.iter(|| jsa(black_box(&input)).expect("Failed to calculate JSA"))
    });

    // CWMA
    group.bench_function(BenchmarkId::new("CWMA", 0), |b| {
        let input = CwmaInput::with_default_candles(&candles);
        b.iter(|| cwma(black_box(&input)).expect("Failed to calculate CWMA"))
    });

    // VPWMA
    group.bench_function(BenchmarkId::new("VPWMA", 0), |b| {
        let input = VpwmaInput::with_default_candles(&candles);
        b.iter(|| vpwma(black_box(&input)).expect("Failed to calculate VPWMA"))
    });

    // SRWMA
    group.bench_function(BenchmarkId::new("SRWMA", 0), |b| {
        let input = SrwmaInput::with_default_candles(&candles);
        b.iter(|| srwma(black_box(&input)).expect("Failed to calculate SRWMA"))
    });

    // SQWMA
    group.bench_function(BenchmarkId::new("SQWMA", 0), |b| {
        let input = SqwmaInput::with_default_candles(&candles);
        b.iter(|| sqwma(black_box(&input)).expect("Failed to calculate SQWMA"))
    });

    // MAAQ
    group.bench_function(BenchmarkId::new("MAAQ", 0), |b| {
        let input = MaaqInput::with_default_candles(&candles);
        b.iter(|| maaq(black_box(&input)).expect("Failed to calculate MAAQ"))
    });

    // MWDX
    group.bench_function(BenchmarkId::new("MWDX", 0), |b| {
        let input = MwdxInput::with_default_candles(&candles);
        b.iter(|| mwdx(black_box(&input)).expect("Failed to calculate MWDX"))
    });

    // NMA
    group.bench_function(BenchmarkId::new("NMA", 0), |b| {
        let input = NmaInput::with_default_candles(&candles);
        b.iter(|| nma(black_box(&input)).expect("Failed to calculate NMA"))
    });

    // EDCF
    group.bench_function(BenchmarkId::new("EDCF", 0), |b| {
        let input = EdcfInput::with_default_candles(&candles);
        b.iter(|| edcf(black_box(&input)).expect("Failed to calculate EDCF"))
    });

    // VWAP
    group.bench_function(BenchmarkId::new("VWAP", 0), |b| {
        let input = VwapInput::with_default_candles(&candles);
        b.iter(|| vwap(black_box(&input)).expect("Failed to calculate VWAP"))
    });

    // HWMA
    group.bench_function(BenchmarkId::new("HWMA", 0), |b| {
        let input = HwmaInput::with_default_candles(&candles);
        b.iter(|| hwma(black_box(&input)).expect("Failed to calculate HWMA"))
    });

    // SWMA
    group.bench_function(BenchmarkId::new("SWMA", 0), |b| {
        let input = SwmaInput::with_default_candles(&candles);
        b.iter(|| swma(black_box(&input)).expect("Failed to calculate SWMA"))
    });

    // TrendFlex
    group.bench_function(BenchmarkId::new("TRENDFLEX", 0), |b| {
        let input = TrendFlexInput::with_default_candles(&candles);
        b.iter(|| trendflex(black_box(&input)).expect("Failed to calculate TRENDFLEX"))
    });

    // VWMA
    group.bench_function(BenchmarkId::new("VWMA", 0), |b| {
        let input = VwmaInput::with_default_candles(&candles);
        b.iter(|| vwma(black_box(&input)).expect("Failed to calculate VWMA"))
    });

    // PWMA
    group.bench_function(BenchmarkId::new("PWMA", 0), |b| {
        let input = PwmaInput::with_default_candles(&candles);
        b.iter(|| pwma(black_box(&input)).expect("Failed to calculate PWMA"))
    });

    // ITREND
    group.bench_function(BenchmarkId::new("ITREND", 0), |b| {
        let input = EhlersITrendInput::with_default_candles(&candles);
        b.iter(|| ehlers_itrend(black_box(&input)).expect("Failed to calculate Ehler's ITrend"))
    });

    // SMMA
    group.bench_function(BenchmarkId::new("SMMA", 0), |b| {
        let input = SmmaInput::with_default_candles(&candles);
        b.iter(|| smma(black_box(&input)).expect("Failed to calculate SMMA"))
    });

    // Reflex
    group.bench_function(BenchmarkId::new("REFLEX", 0), |b| {
        let input = ReflexInput::with_default_candles(&candles);
        b.iter(|| reflex(black_box(&input)).expect("Failed to calculate REFLEX"))
    });

    // JMA
    group.bench_function(BenchmarkId::new("JMA", 0), |b| {
        let input = JmaInput::with_default_candles(&candles);
        b.iter(|| jma(black_box(&input)).expect("Failed to calculate JMA"))
    });

    // High Pass 2 Pole
    group.bench_function(BenchmarkId::new("HIGHPASS_2Pole", 0), |b| {
        let input = HighPass2Input::with_default_candles(&candles);
        b.iter(|| highpass_2_pole(black_box(&input)).expect("Failed to calculate HIGHPASS2"))
    });

    // High Pass
    group.bench_function(BenchmarkId::new("HIGHPASS_1Pole", 0), |b| {
        let input = HighPassInput::with_default_candles(&candles);
        b.iter(|| highpass(black_box(&input)).expect("Failed to calculate HIGHPASS"))
    });

    // Gaussian
    group.bench_function(BenchmarkId::new("GAUSSIAN", 0), |b| {
        let input = GaussianInput::with_default_candles(&candles);
        b.iter(|| gaussian(black_box(&input)).expect("Failed to calculate GAUSSIAN"))
    });

    // Super Smoother 3 Pole
    group.bench_function(BenchmarkId::new("SUPERSMOOTHER3POLE", 0), |b| {
        let input = SuperSmoother3PoleInput::with_default_candles(&candles);
        b.iter(|| {
            supersmoother_3_pole(black_box(&input)).expect("Failed to calculate SUPERSMOOTHER3POLE")
        })
    });

    // Super Smoother
    group.bench_function(BenchmarkId::new("SUPERSMOOTHER", 0), |b| {
        let input = SuperSmootherInput::with_default_candles(&candles);
        b.iter(|| supersmoother(black_box(&input)).expect("Failed to calculate SUPERSMOOTHER"))
    });

    // SinWMA
    group.bench_function(BenchmarkId::new("SINWMA", 0), |b| {
        let input = SinWmaInput::with_default_candles(&candles);
        b.iter(|| sinwma(black_box(&input)).expect("Failed to calculate SINWMA"))
    });

    // Wilders
    group.bench_function(BenchmarkId::new("WILDERS", 0), |b| {
        let input = WildersInput::with_default_candles(&candles);
        b.iter(|| wilders(black_box(&input)).expect("Failed to calculate WILDERS"))
    });

    // Linear Regression
    group.bench_function(BenchmarkId::new("LINREG", 0), |b| {
        let input = LinRegInput::with_default_candles(&candles);
        b.iter(|| linreg(black_box(&input)).expect("Failed to calculate LINREG"))
    });

    // HMA
    group.bench_function(BenchmarkId::new("HMA", 0), |b| {
        let input = HmaInput::with_default_candles(&candles);
        b.iter(|| hma(black_box(&input)).expect("Failed to calculate HMA"))
    });

    // FWMA
    group.bench_function(BenchmarkId::new("FWMA", 0), |b| {
        let input = FwmaInput::with_default_candles(&candles);
        b.iter(|| fwma(black_box(&input)).expect("Failed to calculate FWMA"))
    });

    // MAMA
    group.bench_function(BenchmarkId::new("MAMA", 0), |b| {
        let input = MamaInput::with_default_candles(&candles);
        b.iter(|| mama(black_box(&input)).expect("Failed to calculate MAMA"))
    });

    // TILSON
    group.bench_function(BenchmarkId::new("TILSON", 0), |b| {
        let input = TilsonInput::with_default_candles(&candles);
        b.iter(|| tilson(black_box(&input)).expect("Failed to calculate T3"))
    });

    // KAMA
    group.bench_function(BenchmarkId::new("KAMA", 0), |b| {
        let input = KamaInput::with_default_candles(&candles);
        b.iter(|| kama(black_box(&input)).expect("Failed to calculate KAMA"))
    });

    // TRIMA
    group.bench_function(BenchmarkId::new("TRIMA", 0), |b| {
        let input = TrimaInput::with_default_candles(&candles);
        b.iter(|| trima(black_box(&input)).expect("Failed to calculate TRIMA"))
    });

    // TEMA
    group.bench_function(BenchmarkId::new("TEMA", 0), |b| {
        let input = TemaInput::with_default_candles(&candles);
        b.iter(|| tema(black_box(&input)).expect("Failed to calculate TEMA"))
    });

    // DEMA
    group.bench_function(BenchmarkId::new("DEMA", 0), |b| {
        let input = DemaInput::with_default_candles(&candles);
        b.iter(|| dema(black_box(&input)).expect("Failed to calculate DEMA"))
    });

    // WMA
    group.bench_function(BenchmarkId::new("WMA", 0), |b| {
        let input = WmaInput::with_default_candles(&candles);
        b.iter(|| wma(black_box(&input)).expect("Failed to calculate WMA"))
    });

    // BANDPASS
    group.bench_function(BenchmarkId::new("BANDPASS", 0), |b| {
        let input = BandPassInput::with_default_candles(&candles);
        b.iter(|| bandpass(black_box(&input)).expect("Failed to calculate BANDPASS"))
    });

    // HIGHPASS
    group.bench_function(BenchmarkId::new("HIGHPASS", 0), |b| {
        let input = HighPassInput::with_default_candles(&candles);
        b.iter(|| highpass(black_box(&input)).expect("Failed to calculate HIGHPASS"))
    });

    // AVGPRICE
    group.bench_function(BenchmarkId::new("AVGPRICE", 0), |b| {
        let input = AvgPriceInput::with_default_candles(&candles);
        b.iter(|| avgprice(&input).expect("Failed to calculate AVGPRICE"))
    });

    // ATR
    group.bench_function(BenchmarkId::new("ATR", 0), |b| {
        let input = AtrInput::with_default_candles(&candles);
        b.iter(|| atr(black_box(&input)).expect("Failed to calculate ATR"))
    });

    // AROONOSC
    group.bench_function(BenchmarkId::new("AROONOSC", 0), |b| {
        let input = AroonOscInput::with_default_candles(&candles);
        b.iter(|| aroon_osc(black_box(&input)).expect("Failed to calculate AROONOSC"))
    });

    // AROON
    group.bench_function(BenchmarkId::new("AROON", 0), |b| {
        let input = AroonInput::with_default_candles(&candles);
        b.iter(|| aroon(black_box(&input)).expect("Failed to calculate AROON"))
    });

    // APO
    group.bench_function(BenchmarkId::new("APO", 0), |b| {
        let input = ApoInput::with_default_candles(&candles);
        b.iter(|| apo(black_box(&input)).expect("Failed to calculate APO"))
    });

    // AO
    group.bench_function(BenchmarkId::new("AO", 0), |b| {
        let input = AoInput::with_default_candles(&candles);
        b.iter(|| ao(black_box(&input)).expect("Failed to calculate AO"))
    });

    // ALMA
    group.bench_function(BenchmarkId::new("ALMA", 0), |b| {
        let input = AlmaInput::with_default_candles(&candles);
        b.iter(|| alma(black_box(&input)).expect("Failed to calculate ALMA"))
    });

    // ADOSC
    group.bench_function(BenchmarkId::new("ADOSC", 0), |b| {
        let input = AdoscInput::with_default_candles(&candles);
        b.iter(|| adosc(black_box(&input)).expect("Failed to calculate ADOSC"))
    });

    // ZLEMA
    group.bench_function(BenchmarkId::new("ZLEMA", 0), |b| {
        let input = ZlemaInput::with_default_candles(&candles);
        b.iter(|| zlema(black_box(&input)).expect("Failed to calculate ZLEMA"))
    });

    // Alligator
    group.bench_function(BenchmarkId::new("ALLIGATOR", 0), |b| {
        let input = AlligatorInput::with_default_candles(&candles);
        b.iter(|| alligator(black_box(&input)).expect("Failed to calculate alligator"))
    });

    // ADXR
    group.bench_function(BenchmarkId::new("ADXR", 0), |b| {
        let input = AdxrInput::with_default_candles(&candles);
        b.iter(|| adxr(black_box(&input)).expect("Failed to calculate ADXR"))
    });

    // ADX
    group.bench_function(BenchmarkId::new("ADX", 0), |b| {
        let input = AdxInput::with_default_candles(&candles);
        b.iter(|| adx(black_box(&input)).expect("Failed to calculate ADX"))
    });

    // SMA
    group.bench_function(BenchmarkId::new("SMA", 0), |b| {
        let input = SmaInput::with_default_candles(&candles);
        b.iter(|| sma(black_box(&input)).expect("Failed to calculate SMA"))
    });

    // EMA
    group.bench_function(BenchmarkId::new("EMA", 0), |b| {
        let input = EmaInput::with_default_candles(&candles);
        b.iter(|| ema(black_box(&input)).expect("Failed to calculate EMA"))
    });

    // RSI
    group.bench_function(BenchmarkId::new("RSI", 0), |b| {
        let input = RsiInput::with_default_candles(&candles);
        b.iter(|| rsi(black_box(&input)).expect("Failed to calculate RSI"))
    });

    // ACOSC
    group.bench_function(BenchmarkId::new("ACOSC", 0), |b| {
        let input = AcoscInput::with_default_candles(&candles);
        b.iter(|| acosc(black_box(&input)).expect("Failed to calculate ACOSC"))
    });

    // AD
    group.bench_function(BenchmarkId::new("AD", 0), |b| {
        let input = AdInput::with_default_candles(&candles);
        b.iter(|| ad(black_box(&input)).expect("Failed to calculate AD"))
    });

    group.finish();
}

criterion_group!(benches, benchmark_indicators);
criterion_main!(benches);
