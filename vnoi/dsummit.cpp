// https://oj.vnoi.info/problem/dsummits

#include <bits/stdc++.h>

using namespace std;
using lint = long long;

const int dx[] = {1, 0, -1, 0}, dy[] = {0, 1, 0, -1};

int H, W, D, h, res, idx, px, py; 
int height[507][507], vis[507][507];

struct point {
    int x, y;
};

bool comp(const point& a, const point& b) {
    return height[a.x][a.y] > height[b.x][b.y];
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    cin >> H >> W >> D;
    vector<point> points;
    for(int i = 0; i < H; i++)
        for(int j = 0; j < W; j++) {
            cin >> h;
            height[i][j] = h;
            points.push_back({i, j});
        }

    sort(points.begin(), points.end(), comp);

    for(int i = 0; i < 507; i++)
        for(int j = 0; j < 507; j++) vis[i][j] = -1;

    for(auto p : points) {
        px = p.x, py = p.y;
        if(vis[px][py] != -1) continue;

        int cnt = 0;
        bool closed = true;
        queue<point> q;
        q.push({px, py});

        while(!q.empty()) {
            int cx = q.front().x, cy = q.front().y;
            q.pop();

            if(vis[cx][cy] != -1) continue;
            vis[cx][cy] = idx;
            if(height[cx][cy] == height[px][py]) cnt++;

            for(int k = 0; k < 4; k++) {
                int nx = cx + dx[k], ny = cy + dy[k];
                
                if(nx < 0 || nx >= H || ny < 0 || ny >= W) continue;
                if(height[nx][ny] <= height[px][py] - D) continue;

                if(vis[nx][ny] == -1)
                    q.push({nx, ny});
                else if(vis[nx][ny] < idx) {
                    closed = false;
                }
            }
        }
        idx++;
        if(closed) res += cnt;
    }

    cout << res << "\n";
    
    return 0;
}
