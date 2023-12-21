#include <iostream>
using namespace std;
int main(){
	string cmd;
	int a = 0;
	while(1){
		cin >> cmd;
		if(cmd == "sus"){
			cout << (char)a << endl;
		}
		else if(cmd == "specjal"){
			a++;
		}
		else if(cmd == "skibiditoilet"){
			cout << "Skibidi toilet goni mnie\nSpiewajac te piosenke\nPomaga mi cameraman\nLecimy ratowac ziemie\nJestesmy niepokonani\n";
		}
		else if(cmd == "zgon"){
			a = 0;
		}
		else if(cmd == "rzyg"){
			a--;
		}
		else if(cmd == "uboot"){
			a += 10;
		}
	}
	return 0;
}
