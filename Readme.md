# SCI Quest Decoder

![Aluminum Mallard](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn44.f3.vi581.li1.ci0.static.png)

This decodes and extracts the graphics from [classic Sierra SCI adventure games](https://sciwiki.sierrahelp.com/index.php/Sierra_SCI_Release_List#SCI0_.28late.29) such as Space Quest 3 and Police Quest 2.

![Plaid](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn60.f1.vi184.li7.animation.png)

To use this, run: `make run` - this will run against the assets of the built-in SCI fangame "New Years Mystery".

![Larry](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn716.f1.vi217.li0.animation.png)
![Patti](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn816.f1.vi219.li0.animation.png)

If you have Space Quest 3 or Police Quest 2, put them in `data/sq3` or `data/pq2` then run `make run-sq3` or `make run-pq2`.

![Robot](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn5.f1.vi88.li1.animation.png)

This supports SCI0 games (EGA), not SCI1 (VGA).

![Alien tourist](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn66.f2.vi278.li4.animation.png)

This generates big uncompressed PNGs. To compress them, install `pngquant` and `apngasm` then run `make compress` to make them all a reasonable size.

![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn128.f3.vi520.li1.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn92.f1.vi222.li2.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn128.f3.vi520.li0.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn92.f1.vi222.li5.animation.png)

## New Years Mystery

![Mystery](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn160.f1.vi50.li1.animation.png)

Includes [Doan Sephim's New Years Mystery](https://sciprogramming.com/fangames.php?action=review&id=3) SCI fangame so you can run this without needing a game. Thankyou so much for allowing this, Ryan!

## Others

![Scuba](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn12.f2.vi353.li0.animation.png)

If you're interested, I also wrote decoders for the following:

![Root Monster](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootMonster.png)

* Sierra AGI: https://github.com/chrishulbert/agi-quest-decoder

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png)

* Lemmings: https://github.com/chrishulbert/digger-decoder

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/raw/main/Dopefish.png)

* Commander Keen: https://github.com/chrishulbert/dopefish-decoder

## References

![Rotating wireframe](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn42.f1.vi126.li1.animation.png)

* https://wiki.scummvm.org/index.php?title=SCI/Specifications
* https://github.com/scummvm/scummvm/tree/master/engines/sci
* https://slattstudio.com
* https://github.com/wjp/freesci-archive
* https://sciwiki.sierrahelp.com/index.php/SCI_Specifications

![Goodbye](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn67.f2.vi279.li4.animation.png)
