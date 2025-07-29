#include <iostream>
#include <fstream>
#include <algorithm>
#include <vector>
#include <map>
#include <set>
#include <unordered_map>
#include<algorithm>
#include <string>
#include<set>
#include <math.h>
#include <stack>

using namespace std;

ifstream fin("test.in");
ofstream fout("test.out");

vector<string> v;
set<pair<int, int>> visits;
int dx[] = { -1,0,1,0 };
int dy[] = { 0,1,0,-1 };
int dir;
int XStart, YStart,Xend,Yend;
map<pair<pair<int, int>, int>, int> dist;

int getvalue(pair<pair<int, int>, int> pos) {
	cout << pos.first.first << " " << pos.first.second << " " << pos.second << endl;
	auto it = dist.find(pos);
	if (it == dist.end())
		return 2000000000;
	return it->second;
}


void dijkstra(pair<pair<int, int>, pair<int,int>> start) {
	set<pair<pair<int, int>, pair<int, int>>> q;
	q.insert(start);
	dist[{{start.second.first, start.second.second}, start.first.second}] = 0;
	while (!q.empty()) {
		auto it = q.begin();
		auto node = *it;
		q.erase(it);
		int x = node.second.first;
		int y = node.second.second;
		int d = node.first.second;
		if (v[x][y] == 'E') {
			cout << dist[{{x, y}, d}]<<" "<<d << endl;
		}
		for (int nextDir = d + 1;nextDir <= d + 3;nextDir++) {
			if (dist.find({ {x,y},(nextDir % 4) }) == dist.end()) {
				dist[{ {x, y}, (nextDir % 4) }] = 2000000000;
			}
			if (dist[{ {x, y}, (nextDir % 4) }] > dist[{ {x, y}, d }] + 1000) {
				q.erase({ { dist[{ {x, y}, (nextDir % 4) }],nextDir % 4 },{x,y} });
				dist[{ {x, y}, (nextDir % 4) }] = dist[{ {x, y}, d }] + 1000;
				q.insert({ { dist[{ {x, y}, (nextDir % 4) }],nextDir % 4 },{x,y} });
			}
		}
		int nx = x + dx[d];
		int ny = y + dy[d];
		if(nx>=0&&nx<v.size() &&ny>=0&&ny<v[0].size() && v[nx][ny] !=  '#')
		if (dist.find({ {nx,ny},d }) == dist.end()) {
			dist[{ {nx, ny}, d }] = 2000000000;
		}
		if (dist[{ {nx, ny}, d }] > dist[{ {x, y}, d }] + 1) {
			q.erase({ { dist[{ {nx, ny}, d }],d }, { nx,ny } });
			dist[{ {nx, ny}, d }] = dist[{ {x, y}, d }] + 1;
			q.insert({ { dist[{ {nx, ny}, d }],d }, { nx,ny } });
		}
	}
}
void reverse(pair<pair<int, int>, int> pos) {
	stack<pair<pair<int, int>, int>> s;
	s.push(pos);

	while (!s.empty()) {
		auto currentPos = s.top();
		s.pop();

		int x = currentPos.first.first;
		int y = currentPos.first.second;
		int Currdir = currentPos.second;
		int current = getvalue({ {x, y}, Currdir });

		// Check bounds and conditions
		if (current == 2000000000 || x < 0 || y < 0 || x >= v.size() || y >= v[0].size()) {
			continue;
		}

		v[x][y] = 'O';

		int prevX = x - dx[Currdir];
		int prevY = y - dy[Currdir];

		// Push all directions (Currdir+1 to Currdir+3) onto the stack
		for (int dirr = Currdir + 1; dirr <= Currdir + 3; dirr++) {
			int newDir = dirr % 4;
			int value = getvalue({ {x, y}, newDir });
			if (value < current) {
				s.push({ {x, y}, newDir });
			}
		}

		// Check the previous position and push it if valid
		if (v[prevX][prevY] != '#') {
			int value = getvalue({ {prevX, prevY}, Currdir });
			if (value < current) {
				s.push({ {prevX, prevY}, Currdir });
			}
		}
	}
}
int main() {
	string s;
	while (fin >> s) {
		v.push_back(s);
	}
	dir = 1;
	for (int i = 0;i < v.size();i++) {
		for (int j = 0;j < v[i].size();j++) {
			if (v[i][j] == 'S') {
				XStart = i;
				YStart = j;
			}
			else if (v[i][j] == 'E') {
				Xend = i;
				Yend = j;
			}
		}
	}
	dijkstra({ {0,dir}, { XStart, YStart } });
	reverse({ {Xend,Yend},0 });
	int nr = 0;
	for (auto it : v) {
		cout << it << endl;
		for (auto it2 : it) {
			if (it2 == 'O') {
				nr++;
			}
		}
	}
	cout << nr;
	
	return 0;
}