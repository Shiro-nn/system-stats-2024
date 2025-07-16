nano /etc/systemd/system/system-stats.service
chmod 500 /etc/system-stats
chmod 500 /etc/systemd/system/system-stats.service

systemctl start system-stats
systemctl enable system-stats --now
systemctl status system-stats


# restart
systemctl restart system-stats